#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

fn hashed_fields_identifer(fields: &[(String, TypeDefinition)]) -> String {
  let mut hasher = DefaultHasher::new();
  fields.hash(&mut hasher);
  format!("{:x}", hasher.finish())
}

fn default_padded() -> bool {
  true
}

#[derive(Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Struct {
  pub identifier: Option<String>,
  #[cfg_attr(feature = "serde", serde(default = "default_padded"))]
  pub padded: bool,
  pub fields: Vec<(String, TypeDefinition)>,
}

impl Struct {
  pub fn new(
    identifier: Option<&str>,
    padded: bool,
    fields: Vec<(String, TypeDefinition)>,
  ) -> Self {
    Self {
      identifier: identifier.map(String::from),
      padded,
      fields,
    }
  }

  pub fn identifier(&self) -> String {
    if let Some(identifier) = &self.identifier {
      identifier.clone()
    } else {
      hashed_fields_identifer(&self.fields)
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
    if self.identifier.is_none() {
      self.typescript_type()
    } else {
      self.identifier().to_pascal_case()
    }
  }

  pub fn into_function_name(&self) -> String {
    format!("__into_{}", self.identifier())
  }

  pub fn from_function_name(&self) -> String {
    format!("__from_{}", self.identifier())
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
}

impl From<Struct> for TypeConverter {
  fn from(r#struct: Struct) -> Self {
    let mut globals = Vec::new();
    let typescript = r#struct.typescript();

    if r#struct.identifier.is_some() {
      globals.push(format!(
        "export interface {} {}",
        typescript,
        r#struct.typescript_type()
      ));
    }

    let mut into_body = Vec::new();
    let mut properties = Vec::new();

    let mut offset = 0;
    let align = r#struct
      .fields()
      .iter()
      .map(|(_, definition, _)| definition.align_of())
      .max()
      .unwrap_or(0);

    for (property, definition, mut descriptor) in r#struct.fields() {
      if r#struct.padded {
        offset += calculate_padding(offset, definition.align_of());
      }

      globals.append(&mut descriptor.converter.globals);

      let accessor = format!("__data.{}", property);

      match definition {
        TypeDefinition::Primitive(ref primitive) => {
          properties.push((
            property,
            descriptor.converter.from.replace(
              "{}",
              &format!(
                "__data_view.{}({})",
                primitive.native.data_view_getter(),
                offset
              ),
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
          properties.push((
            property,
            descriptor.converter.from.replace(
              "{}",
              &format!(
                "new Deno.UnsafePointer(__data_view.getBigUint64({}))",
                offset
              ),
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

          properties.push((
            property,
            if let BufferType::None = buffer.ty {
              source_buffer
            } else {
              format!("new {}({})", buffer.ty.typed_array(), source_buffer)
            },
          ));

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
          properties.push((
            property,
            descriptor.converter.from.replace(
              "{}",
              &format!(
                "__array_buffer.slice({}, {})",
                offset,
                offset + definition.size_of()
              ),
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
      + if r#struct.padded && offset != 0 {
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
        return {{\n{}\n}};\n\
      }}",
      r#struct.from_function_name(),
      typescript,
      properties.iter().map(|(property, value)| format!("  {}: {}", property, value)).collect::<Vec<String>>().join(",\n"),
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
      r#struct.into_function_name(),
      typescript,
      size,
      into_body.join("\n")
    ));

    TypeConverter {
      globals,
      typescript,
      into: format!("{}({{}})", r#struct.into_function_name()),
      from: format!("{}({{}})", r#struct.from_function_name()),
    }
  }
}

impl From<Struct> for TypeDescriptor {
  fn from(r#struct: Struct) -> Self {
    TypeDescriptor {
      native: NativeType::Pointer,
      converter: r#struct.into(),
    }
  }
}
