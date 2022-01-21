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

fn hashed_fields_identifer(fields: &[TypeDefinition]) -> String {
  let mut hasher = DefaultHasher::new();
  fields.hash(&mut hasher);
  format!("{:x}", hasher.finish())
}

#[derive(Clone, Hash)]
pub struct Tuple {
  pub identifier: String,
  pub anonymous: bool,
  pub fields: Vec<TypeDefinition>,
}

impl Tuple {
  pub fn new(identifier: Option<&str>, fields: Vec<TypeDefinition>) -> Self {
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
      "[{}]",
      self
        .fields()
        .iter()
        .map(|(_, descriptor)| { descriptor.converter.typescript.clone() })
        .collect::<Vec<String>>()
        .join(", ")
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

  pub fn fields(&self) -> Vec<(TypeDefinition, TypeDescriptor)> {
    self
      .fields
      .clone()
      .into_iter()
      .map(|definition| (definition.clone(), TypeDescriptor::from(definition)))
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
      .map(|(definition, _)| definition.align_of())
      .max()
      .unwrap_or(0);

    for (field, (definition, mut descriptor)) in
      self.fields().into_iter().enumerate()
    {
      offset += calculate_padding(offset, definition.align_of());
      globals.append(&mut descriptor.converter.globals);

      let accessor = format!("__data[{}]", field);

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

  pub fn generate_from_function(&self, _globals: &mut Vec<String>) {}
}

impl From<Tuple> for TypeDescriptor {
  fn from(tuple: Tuple) -> Self {
    let mut globals = Vec::new();

    if !tuple.anonymous {
      globals.push(format!(
        "export type {} = {};",
        tuple.typescript(),
        tuple.typescript_type()
      ));
    }

    tuple.generate_into_function(&mut globals);
    tuple.generate_from_function(&mut globals);

    TypeDescriptor {
      native: NativeType::Pointer,
      converter: TypeConverter {
        globals,
        typescript: tuple.typescript(),
        into: format!("{}({{}})", tuple.into_function_name()),
        from: format!("{}({{}})", tuple.from_function_name()),
      },
    }
  }
}
