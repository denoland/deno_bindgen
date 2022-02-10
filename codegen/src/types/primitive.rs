#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::NativeType;
use super::TypeConverter;
use super::TypeDescriptor;

#[derive(Clone, Hash)]
#[cfg_attr(
  feature = "serde",
  derive(Serialize, Deserialize)
)]
pub struct Primitive {
  pub native: NativeType,
}

impl Primitive {
  pub fn new(native: NativeType) -> Self {
    Self { native }
  }
}

impl From<Primitive> for TypeDescriptor {
  fn from(primitive: Primitive) -> Self {
    let typescript = match primitive.native {
      NativeType::Void => "void",
      NativeType::U8
      | NativeType::I8
      | NativeType::U16
      | NativeType::I16
      | NativeType::U32
      | NativeType::I32
      | NativeType::F32
      | NativeType::F64 => "number",
      NativeType::U64
      | NativeType::I64
      | NativeType::USize
      | NativeType::ISize => "bigint",
      NativeType::Pointer => "Deno.UnsafePointer",
    };
    let converter = TypeConverter {
      globals: Vec::new(),
      typescript: typescript.to_string(),
      into: "{}".to_string(),
      from: if let NativeType::Pointer = primitive.native {
        "new Deno.UnsafePointer({})"
      } else {
        "{}"
      }
      .to_string(),
    };

    TypeDescriptor {
      native: primitive.native,
      converter,
    }
  }
}
