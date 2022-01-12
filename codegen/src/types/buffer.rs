use super::BufferType;
use super::NativeType;
use super::TypeConverter;
use super::TypeConverters;
use super::TypeDescriptor;

#[derive(Clone)]
pub struct Buffer {
  pub r#type: BufferType,
  pub length: Option<usize>,
}

impl Buffer {
  pub fn new(r#type: BufferType, length: Option<usize>) -> Self {
    Self { r#type, length }
  }
}

impl From<Buffer> for TypeDescriptor {
  fn from(buffer: Buffer) -> Self {
    if let Some(length) = buffer.length {
      let converters = if let BufferType::None = buffer.r#type {
        TypeConverters {
          into: TypeConverter {
            typescript: "ArrayBuffer".to_string(),
            global: None,
            local: None,
            inline: format!("new Uint8Array({{}} {})", length),
          },
          from: TypeConverter {
            typescript: "ArrayBuffer".to_string(),
            global: None,
            local: None,
            inline: format!("{{}}.getArrayBuffer({})", length),
          },
        }
      } else {
        let constructor: String = buffer.r#type.into();
        TypeConverters {
          into: TypeConverter {
            typescript: constructor.to_string(),
            global: None,
            local: None,
            inline: format!(
              "Deno.UnsafePointer.of(new {}({{}}.buffer {}))",
              constructor, length
            ),
          },
          from: TypeConverter {
            typescript: constructor.to_string(),
            global: None,
            local: None,
            inline: format!(
              "new {}(new Deno.UnsafePointerView({{}}).getArrayBuffer({}))",
              constructor, length
            ),
          },
        }
      };

      TypeDescriptor {
        native: NativeType::Pointer,
        converters,
      }
    } else {
      let constructor: String = buffer.r#type.into();
      TypeDescriptor {
        native: NativeType::Pointer,
        converters: TypeConverters {
          into: TypeConverter {
            typescript: constructor.to_string(),
            global: None,
            local: None,
            inline: format!(
              "Deno.UnsafePointer.of(new {}({{}}.buffer))",
              constructor
            ),
          },
          from: TypeConverter {
            typescript: "Deno.UnsafePointer".to_string(),
            global: None,
            local: None,
            inline: "{}".to_string(),
          },
        },
      }
    }
  }
}
