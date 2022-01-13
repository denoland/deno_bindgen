use super::BufferType;
use super::NativeType;
use super::TypeConverter;
use super::TypeConverters;
use super::TypeDescriptor;

#[derive(Clone)]
pub struct Buffer {
  pub r#type: BufferType,
  pub length: usize,
}

impl Buffer {
  pub fn new(r#type: BufferType, length: usize) -> Self {
    Self { r#type, length }
  }
}

impl From<Buffer> for TypeDescriptor {
  fn from(buffer: Buffer) -> Self {
    let converters = if let BufferType::None = buffer.r#type {
      TypeConverters {
        global: None,
        typescript: "ArrayBuffer".to_string(),
        into: TypeConverter {
          local: None,
          inline: format!("new Uint8Array({{}}, {})", buffer.length),
        },
        from: TypeConverter {
          local: None,
          inline: format!("{{}}.getArrayBuffer({})", buffer.length),
        },
      }
    } else {
      let constructor: String = buffer.r#type.into();
      TypeConverters {
        global: None,
        typescript: constructor.to_string(),
        into: TypeConverter {
          local: None,
          inline: format!(
            "Deno.UnsafePointer.of(new {}({{}}.buffer, {}))",
            constructor, buffer.length
          ),
        },
        from: TypeConverter {
          local: None,
          inline: format!(
            "new {}(new Deno.UnsafePointerView({{}}).getArrayBuffer({}))",
            constructor, buffer.length
          ),
        },
      }
    };

    TypeDescriptor {
      native: NativeType::Pointer,
      converters,
    }
  }
}
