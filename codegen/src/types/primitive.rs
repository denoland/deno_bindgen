use super::{NativeType, TypeConverter, TypeConverters, TypeDescriptor};

#[derive(Clone)]
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
      | NativeType::U64
      | NativeType::I64
      | NativeType::USize
      | NativeType::ISize
      | NativeType::F32
      | NativeType::F64 => "number",
      NativeType::Pointer => "Deno.UnsafePointer",
    };
    let converters = TypeConverters {
      into: TypeConverter {
        typescript: typescript.to_string(),
        global: None,
        local: None,
        inline: "{}".to_string(),
      },
      from: TypeConverter {
        typescript: typescript.to_string(),
        global: None,
        local: None,
        inline: "{}".to_string(),
      },
    };

    TypeDescriptor {
      native: primitive.native,
      converters,
    }
  }
}
