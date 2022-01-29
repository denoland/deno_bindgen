use super::BufferType;
use super::NativeType;
use super::TypeConverter;
use super::TypeDescriptor;

#[derive(Clone, Hash)]
pub struct Buffer {
  pub ty: BufferType,
  pub length: usize,
}

impl Buffer {
  pub fn new(ty: BufferType, length: usize) -> Self {
    Self { ty, length }
  }
}

impl From<Buffer> for TypeDescriptor {
  fn from(buffer: Buffer) -> Self {
    let converter = if let BufferType::None = buffer.ty {
      TypeConverter {
        globals: Vec::new(),
        typescript: "ArrayBuffer".to_string(),
        into: format!("new Uint8Array({{}}, {})", buffer.length),
        from: format!("{{}}.getArrayBuffer({})", buffer.length),
      }
    } else {
      let constructor = buffer.ty.typed_array();
      TypeConverter {
        globals: Vec::new(),
        typescript: constructor.to_string(),
        into: format!(
          "Deno.UnsafePointer.of(new {}({{}}.buffer, {}))",
          constructor, buffer.length
        ),
        from: format!(
          "new {}(new Deno.UnsafePointerView({{}}).getArrayBuffer({}))",
          constructor, buffer.length
        ),
      }
    };

    TypeDescriptor {
      native: NativeType::Pointer,
      converter,
    }
  }
}
