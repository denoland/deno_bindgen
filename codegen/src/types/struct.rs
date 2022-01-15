use std::collections::HashSet;
use std::iter::FromIterator;

use inflector::Inflector;

use super::BufferType;
use super::NativeType;
use super::TypeConverter;
use super::TypeDefinition;
use super::TypeDescriptor;

fn calculate_padding(offset: usize, alignment: usize) -> usize {
  let misalignment = offset % alignment;
  if misalignment > 0 {
    alignment - misalignment
  } else {
    0
  }
}

#[derive(Clone)]
pub struct StructLayout {
  pub fields: Vec<(String, TypeDefinition)>,
}

#[derive(Clone)]
pub struct Struct {
  pub name: String,
  pub layout: StructLayout,
}

impl Struct {
  pub fn new(name: &str, layout: StructLayout) -> Self {
    Self {
      name: name.to_string(),
      layout,
    }
  }

  pub fn type_name(&self) -> String {
    self.name.to_pascal_case()
  }

  pub fn variable_name(&self) -> String {
    format!("__{}", self.name.to_snake_case())
  }

  pub fn into_function_name(&self) -> String {
    format!("{}_into", self.variable_name())
  }

  pub fn from_function_name(&self) -> String {
    format!("{}_from", self.variable_name())
  }

  pub fn fields(&self) -> Vec<(String, TypeDefinition, TypeDescriptor)> {
    self
      .layout
      .clone()
      .fields
      .into_iter()
      .map(|(property, definition)| {
        (
          property,
          definition.clone(),
          TypeDescriptor::from(definition),
        )
      })
      .collect()
  }

  pub fn generate_into_function(&self) -> String {
    let mut views_required: HashSet<String> = HashSet::new();
    views_required
      .insert("const __u8_view = new Uint8Array(__array_buffer);".to_string());

    let mut body = Vec::new();
    let mut offset = 0;
    let align = self
      .fields()
      .iter()
      .map(|(_, definition, _)| definition.align_of())
      .max()
      .unwrap_or(0);

    for (property, definition, descriptor) in self.fields() {
      offset += calculate_padding(offset, definition.align_of());

      println!("{}, {}, {}", offset, definition.size_of(), definition.align_of());

      match definition {
        TypeDefinition::Primitive(ref primitive) => {
          let view_constructor =
            String::from(BufferType::from(primitive.native));
          let view_variable = match primitive.native {
            NativeType::U8 => "__u8_view",
            NativeType::I8 => "__i8_view",
            NativeType::U16 => "__u16_view",
            NativeType::I16 => "__i16_view",
            NativeType::U32 => "__u32_view",
            NativeType::I32 => "__i32_view",
            NativeType::Pointer | NativeType::U64 | NativeType::USize => {
              "__u64_view"
            }
            NativeType::I64 | NativeType::ISize => "__i64_view",
            NativeType::F32 => "__f32_view",
            NativeType::F64 => "__f64_view",
            _ => panic!("Unsupported type"),
          };

          views_required.insert(format!(
            "const {} = new {}(__array_buffer);",
            view_variable, view_constructor
          ));

          body.push(format!(
            "{}[{}] = {};",
            view_variable,
            offset / definition.size_of(),
            descriptor.converter.into.replace(
              "{}",
              &format!(
                "{}.{}{}",
                self.variable_name(),
                property,
                if let NativeType::Pointer = primitive.native {
                  ".value"
                } else {
                  ""
                }
              )
            )
          ));
        }
        TypeDefinition::Pointer(_) => {
          views_required.insert(
            "const __u64_view = new BigUint64Array(__array_buffer);"
              .to_string(),
          );

          body.push(format!(
            "__u64_view[{}] = {}.value;",
            offset / definition.size_of(),
            descriptor.converter.into.replace(
              "{}",
              &format!("{}.{}", self.variable_name(), property,)
            )
          ));
        }
        TypeDefinition::Buffer(ref buffer) => {
          let accessor = format!("{}.{}", self.variable_name(), property);
          let source = if let BufferType::None = buffer.r#type {
            format!("new Uint8Array({})", accessor)
          } else if let BufferType::U8 = buffer.r#type {
            accessor
          } else {
            format!("new Uint8Array({}.buffer)", accessor)
          };

          body.push(format!(
            "__u8_view.set({}, {});",
            source,
            offset,
          ));
        }
        TypeDefinition::CString => unimplemented!(),
        TypeDefinition::Struct(_) => unimplemented!(),
      }

      offset += definition.size_of();
    }

    let size = offset + calculate_padding(offset, align);

    format!(
      "function {}({}: {}): Uint8Array {{\n\
        const __array_buffer = new ArrayBuffer({});\n\
        {}\n\
        {}\n\
        return new Uint8Array(__array_buffer);\n\
      }}",
      self.into_function_name(),
      self.variable_name(),
      self.type_name(),
      size,
      Vec::from_iter(views_required).join("\n"),
      body.join("\n")
    )
  }

  pub fn generate_from_function(&self) -> String {
    String::new()
  }
}

impl From<Struct> for TypeDescriptor {
  fn from(r#struct: Struct) -> Self {
    let mut typescript_properties: Vec<String> = Vec::new();

    for (property, _, descriptor) in r#struct.fields() {
      typescript_properties.push(format!(
        "{}: {};\n",
        property, descriptor.converter.typescript
      ));
    }

    let typescript_interface = format!(
      "export interface {} {{\n{}}}",
      r#struct.type_name(),
      typescript_properties.join("")
    );

    TypeDescriptor {
      native: NativeType::Pointer,
      converter: TypeConverter {
        global: Some(format!(
          "{}\n{}\n{}",
          typescript_interface,
          r#struct.generate_into_function(),
          r#struct.generate_from_function()
        )),
        typescript: r#struct.type_name(),
        into: format!("{}({{}})", r#struct.into_function_name()),
        from: format!("{}({{}})", r#struct.from_function_name()),
      },
    }
  }
}
