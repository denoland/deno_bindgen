use inflector::Inflector;

use super::BufferType;
use super::NativeType;
use super::TypeConverter;
use super::TypeDefinition;
use super::TypeDescriptor;

pub trait Sizeof {
  fn byte_size(&self) -> usize;
}

#[derive(Clone)]
pub struct StructLayout {
  pub size: usize,
  pub align: usize,
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
    let mut views_required: Vec<String> = Vec::new();
    let mut body = Vec::new();
    let mut offset = 0;

    views_required.push(format!(
      "const __array_buffer = new ArrayBuffer({});",
      self.layout.size
    ));

    for (property, definition, descriptor) in self.fields() {
      match definition {
        TypeDefinition::Primitive(primitive) => {
          if let NativeType::Pointer = primitive.native {
            unimplemented!()
          } else {
            let view_constructor =
              String::from(BufferType::from(primitive.native));
            let view_variable = match primitive.native {
              NativeType::Void | NativeType::Pointer => unreachable!(),
              NativeType::U8 => "__u8_view",
              NativeType::I8 => "__i8_view",
              NativeType::U16 => "__u16_view",
              NativeType::I16 => "__i16_view",
              NativeType::U32 => "__u32_view",
              NativeType::I32 => "__i32_view",
              NativeType::U64 => "__u64_view",
              NativeType::I64 => "__i64_view",
              NativeType::USize => "__u64_view",
              NativeType::ISize => "__i64_view",
              NativeType::F32 => "__f32_view",
              NativeType::F64 => "__f64_view",
            };

            views_required.push(format!(
              "const {} = new {}(__array_buffer);",
              view_variable, view_constructor
            ));

            body.push(format!(
              "{}[{}] = {};",
              view_variable,
              offset,
              descriptor.converter.into.replace(
                "{}",
                &format!("{}.{}", self.variable_name(), property)
              )
            ));
          }
        }
        TypeDefinition::Pointer(_) => unimplemented!(),
        TypeDefinition::Buffer(_) => unimplemented!(),
        TypeDefinition::CString => unimplemented!(),
        TypeDefinition::Struct(_) => unimplemented!(),
      }

      offset += descriptor.native.size();
    }

    views_required.dedup();

    format!(
      "function {}({}: {}): Deno.UnsafePointer {{\n{}{}\nreturn new Deno.UnsafePointer(new Uint8Array(__array_buffer));\n}}",
      self.into_function_name(), self.variable_name(), self.type_name(), views_required.join("\n"), body.join("\n")
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
          "{}\n{}\n{}\n",
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
