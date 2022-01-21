use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::iter::FromIterator;

use indexmap::IndexSet;
use inflector::Inflector;

use super::calculate_padding;
use super::BufferType;
use super::NativeType;
use super::TypeConverter;
use super::TypeDefinition;
use super::TypeDescriptor;

fn hashed_fields_identifer(fields: &[(String, TypeDefinition)]) -> String {
  let mut hasher = DefaultHasher::new();
  fields.hash(&mut hasher);
  format!("{:x}", hasher.finish())
}

#[derive(Clone, Hash)]
pub struct Struct {
  pub identifier: String,
  pub anonymous: bool,
  pub fields: Vec<(String, TypeDefinition)>,
}

impl Struct {
  pub fn new(
    identifier: Option<&str>,
    fields: Vec<(String, TypeDefinition)>,
  ) -> Self {
    Self {
      identifier: identifier
        .map(String::from)
        .unwrap_or_else(|| hashed_fields_identifer(&fields)),
      anonymous: identifier.is_none(),
      fields,
    }
  }

  pub fn typescript_type(&self) -> String {
    format!(
      "{{\n{}\n}}",
      self
        .fields()
        .iter()
        .map(|(property, _, descriptor)| {
          format!("{}: {};", property, descriptor.converter.typescript)
        })
        .collect::<Vec<String>>()
        .join("\n")
    )
  }

  pub fn typescript(&self) -> String {
    if self.anonymous {
      self.typescript_type()
    } else {
      self.identifier.to_pascal_case()
    }
  }

  pub fn into_function_name(&self) -> String {
    format!("__into_{}", self.identifier)
  }

  pub fn from_function_name(&self) -> String {
    format!("__from_{}", self.identifier)
  }

  pub fn fields(&self) -> Vec<(String, TypeDefinition, TypeDescriptor)> {
    self
      .fields
      .clone()
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

  pub fn generate_into_function(&self, globals: &mut Vec<String>) {
    let mut views: IndexSet<String> = IndexSet::new();
    views
      .insert("const __u8_view = new Uint8Array(__array_buffer);".to_string());

    let mut body = Vec::new();
    let mut offset = 0;
    let align = self
      .fields()
      .iter()
      .map(|(_, definition, _)| definition.align_of())
      .max()
      .unwrap_or(0);

    for (property, definition, mut descriptor) in self.fields() {
      offset += calculate_padding(offset, definition.align_of());
      globals.append(&mut descriptor.converter.globals);

      let accessor = format!("__data.{}", property);

      match definition {
        TypeDefinition::Primitive(ref primitive) => {
          let view_constructor =
            BufferType::from(primitive.native).typed_array();
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

          views.insert(format!(
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
                "{}{}",
                accessor,
                if let NativeType::Pointer = primitive.native {
                  ".value"
                } else {
                  ""
                }
              )
            )
          ));
        }
        TypeDefinition::CString | TypeDefinition::Pointer(_) => {
          views.insert(
            "const __u64_view = new BigUint64Array(__array_buffer);"
              .to_string(),
          );

          body.push(format!(
            "__u64_view[{}] = {}.value;",
            offset / definition.size_of(),
            descriptor.converter.into.replace("{}", &accessor)
          ));
        }
        TypeDefinition::Buffer(ref buffer) => {
          let source = if let BufferType::None = buffer.r#type {
            format!("new Uint8Array({})", accessor)
          } else if let BufferType::U8 = buffer.r#type {
            accessor
          } else {
            format!("new Uint8Array({}.buffer)", accessor)
          };

          body.push(format!("__u8_view.set({}, {});", source, offset));
        }
        TypeDefinition::Tuple(_) | TypeDefinition::Struct(_) => {
          body.push(format!(
            "__u8_view.set(new Uint8Array({}.buffer), {});",
            descriptor.converter.into.replace("{}", &accessor),
            offset,
          ));
        }
      }
      offset += definition.size_of();
    }

    let size = offset + calculate_padding(offset, align);

    globals.push(format!(
      "function {}(__data: {}): Uint8Array {{\n\
        const __array_buffer = new ArrayBuffer({});\n\
        {}\n\
        {}\n\
        return __u8_view;\n\
      }}",
      self.into_function_name(),
      self.typescript(),
      size,
      Vec::from_iter(views).join("\n"),
      body.join("\n")
    ));
  }

  pub fn generate_from_function(&self, globals: &mut Vec<String>) {
    let mut properties = Vec::new();
    let mut body: Vec<String> = Vec::new();
    let mut views: IndexSet<String> = IndexSet::new();
    let mut offset = 0;
    let align = self
      .fields()
      .iter()
      .map(|(_, definition, _)| definition.align_of())
      .max()
      .unwrap_or(0);

    for (property, definition, mut descriptor) in self.fields() {
      offset += calculate_padding(offset, definition.align_of());
      globals.append(&mut descriptor.converter.globals);

      let variable_name = format!("__{}", property);
      properties.push((property, variable_name.clone()));

      match definition {
        TypeDefinition::Primitive(ref primitive) => {
          let view_constructor =
            BufferType::from(primitive.native).typed_array();
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

          views.insert(format!(
            "const {} = new {}(__array_buffer);",
            view_variable, view_constructor
          ));

          body.push(format!(
            "const {} = {};",
            variable_name,
            descriptor.converter.from.replace(
              "{}",
              &format!("{}[{}]", view_variable, offset / definition.size_of())
            ),
          ));
        }

        TypeDefinition::CString | TypeDefinition::Pointer(_) => {
          views.insert(
            "const __u64_view = new BigUint64Array(__array_buffer);"
              .to_string(),
          );

          body.push(format!(
            "const {} = {};",
            variable_name,
            descriptor.converter.from.replace(
              "{}",
              &format!(
                "new Deno.UnsafePointer(__u64_view[{}])",
                offset / definition.size_of()
              )
            ),
          ));
        }
        TypeDefinition::Buffer(ref buffer) => {
          let constructor = buffer.r#type.typed_array();
          let array_buffer = format!(
            "__array_buffer.slice({}, {})",
            offset,
            offset + definition.size_of()
          );

          body.push(format!(
            "const {} = {};",
            variable_name,
            if let BufferType::None = buffer.r#type {
              array_buffer
            } else {
              format!("new {}({})", constructor, array_buffer)
            }
          ));
        }
        TypeDefinition::Tuple(_) | TypeDefinition::Struct(_) => {
          body.push(format!(
            "const {} = {};",
            variable_name,
            descriptor.converter.from.replace(
              "{}",
              &format!(
                "__array_buffer.slice({}, {})",
                offset,
                offset + definition.size_of()
              )
            ),
          ));
        }
      }

      offset += definition.size_of();
    }

    let size = offset + calculate_padding(offset, align);

    globals.push(format!(
      "function {}(__source: ArrayBuffer | Uint8Array | Deno.UnsafePointer | Deno.UnsafePointerView): {} {{\n\
        const __array_buffer =\n\
          (__source instanceof ArrayBuffer\n\
            ? __source\n\
            : __source instanceof Uint8Array\n\
            ? __source.buffer\n\
            : __source instanceof Deno.UnsafePointer\n\
            ? new Deno.UnsafePointerView(__source).getArrayBuffer({size})\n\
            : __source instanceof Deno.UnsafePointerView\n\
            ? __source.getArrayBuffer({size})\n\
            : undefined)!;\n\
        {}\n\
        {}\n\
        return {{\n{}\n}};\n\
      }}",
      self.from_function_name(),
      self.typescript(),
      Vec::from_iter(views).join("\n"),
      body.join("\n"),
      properties.iter().map(|(property, variable_name)| format!("  {}: {}", property, variable_name)).collect::<Vec<String>>().join(",\n"),
      size = size,
    ));
  }
}

impl From<Struct> for TypeDescriptor {
  fn from(r#struct: Struct) -> Self {
    let mut globals = Vec::new();

    if !r#struct.anonymous {
      globals.push(format!(
        "export interface {} {}",
        r#struct.typescript(),
        r#struct.typescript_type()
      ));
    }

    r#struct.generate_into_function(&mut globals);
    r#struct.generate_from_function(&mut globals);

    TypeDescriptor {
      native: NativeType::Pointer,
      converter: TypeConverter {
        globals,
        typescript: r#struct.typescript(),
        into: format!("{}({{}})", r#struct.into_function_name()),
        from: format!("{}({{}})", r#struct.from_function_name()),
      },
    }
  }
}
