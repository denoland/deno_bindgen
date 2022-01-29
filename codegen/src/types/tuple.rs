use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

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
  pub padded: bool,
  pub fields: Vec<TypeDefinition>,
}

impl Tuple {
  pub fn new(
    identifier: Option<&str>,
    padded: bool,
    fields: Vec<TypeDefinition>,
  ) -> Self {
    Self {
      identifier: identifier
        .map(String::from)
        .unwrap_or_else(|| hashed_fields_identifer(&fields)),
      anonymous: identifier.is_none(),
      padded,
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
}

impl From<Tuple> for TypeConverter {
  fn from(tuple: Tuple) -> Self {
    let mut globals = Vec::new();
    let typescript = tuple.typescript();

    if !tuple.anonymous {
      globals.push(format!(
        "export type {} = {};",
        typescript,
        tuple.typescript_type()
      ));
    }

    let mut into_body = Vec::new();
    let mut properties = Vec::new();

    let mut offset = 0;
    let align = tuple
      .fields()
      .iter()
      .map(|(definition, _)| definition.align_of())
      .max()
      .unwrap_or(0);

    for (field, (definition, mut descriptor)) in
      tuple.fields().into_iter().enumerate()
    {
      if tuple.padded {
        offset += calculate_padding(offset, definition.align_of());
      }

      globals.append(&mut descriptor.converter.globals);

      let accessor = format!("__data[{}]", field);

      match definition {
        TypeDefinition::Primitive(ref primitive) => {
          properties.push(descriptor.converter.from.replace(
            "{}",
            &format!(
              "__data_view.{}({})",
              primitive.native.data_view_getter(),
              offset
            ),
          ));

          into_body.push(format!(
            "__data_view.{}({}, {});",
            primitive.native.data_view_setter(),
            offset,
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
          properties.push(descriptor.converter.from.replace(
            "{}",
            &format!(
              "new Deno.UnsafePointer(__data_view.getBigUint64({}))",
              offset
            ),
          ));

          into_body.push(format!(
            "__data_view.setBigUint64({}, {}.value);",
            offset,
            descriptor.converter.into.replace("{}", &accessor)
          ));
        }
        TypeDefinition::Buffer(ref buffer) => {
          let source_buffer = format!(
            "__array_buffer.slice({}, {})",
            offset,
            offset + definition.size_of()
          );

          properties.push(if let BufferType::None = buffer.ty {
            source_buffer
          } else {
            format!("new {}({})", buffer.ty.typed_array(), source_buffer)
          });

          into_body.push(format!(
            "__u8_array.set({}, {});",
            if let BufferType::None = buffer.ty {
              format!("new Uint8Array({})", accessor)
            } else if let BufferType::U8 = buffer.ty {
              accessor
            } else {
              format!("new Uint8Array({}.buffer)", accessor)
            },
            offset
          ));
        }
        TypeDefinition::Tuple(_) | TypeDefinition::Struct(_) => {
          properties.push(descriptor.converter.from.replace(
            "{}",
            &format!(
              "__array_buffer.slice({}, {})",
              offset,
              offset + definition.size_of()
            ),
          ));

          into_body.push(format!(
            "__u8_array.set(new Uint8Array({}.buffer), {});",
            descriptor.converter.into.replace("{}", &accessor),
            offset,
          ));
        }
      }

      offset += definition.size_of();
    }

    let size = offset
      + if tuple.padded {
        calculate_padding(offset, align)
      } else {
        0
      };

    // from function
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
        const __data_view = new DataView(__array_buffer);\n\
        return [\n{}\n];\n\
      }}",
      tuple.from_function_name(),
      typescript,
      properties.join(",\n"),
      size = size,
    ));

    // into function
    globals.push(format!(
      "function {}(__data: {}): Uint8Array {{\n\
        const __array_buffer = new ArrayBuffer({});\n\
        const __u8_array = new Uint8Array(__array_buffer);
        const __data_view = new DataView(__array_buffer);\n\
        {}\n\
        return __u8_array;\n\
      }}",
      tuple.into_function_name(),
      typescript,
      size,
      into_body.join("\n")
    ));

    TypeConverter {
      globals,
      typescript,
      into: format!("{}({{}})", tuple.into_function_name()),
      from: format!("{}({{}})", tuple.from_function_name()),
    }
  }
}

impl From<Tuple> for TypeDescriptor {
  fn from(tuple: Tuple) -> Self {
    TypeDescriptor {
      native: NativeType::Pointer,
      converter: tuple.into(),
    }
  }
}
