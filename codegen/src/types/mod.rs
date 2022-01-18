use self::buffer::Buffer;
use self::cstring::CString;
use self::pointer::Pointer;
use self::primitive::Primitive;
use self::r#struct::Struct;

pub mod buffer;
pub mod cstring;
pub mod pointer;
pub mod primitive;
pub mod r#struct;

fn calculate_padding(offset: usize, alignment: usize) -> usize {
  let misalignment = offset % alignment;
  if misalignment > 0 {
    alignment - misalignment
  } else {
    0
  }
}

#[derive(Clone, Copy)]
pub enum NativeType {
  Void,
  U8,
  I8,
  U16,
  I16,
  U32,
  I32,
  U64,
  I64,
  USize,
  ISize,
  F32,
  F64,
  Pointer,
}

#[derive(Clone, Copy)]
pub enum BufferType {
  None,
  U8,
  I8,
  U16,
  I16,
  U32,
  I32,
  U64,
  I64,
  USize,
  ISize,
  F32,
  F64,
}

#[derive(Clone)]
pub enum TypeDefinition {
  Primitive(Primitive),
  Pointer(Pointer),
  Buffer(Buffer),
  CString,
  Struct(Struct),
  //  Tuple(Vec<TypeDefinition>),
  //  Enum(Vec<(String, Option<TypeDefinition>)>),
  //  Array(Vec<TypeDefinition>),
}

impl TypeDefinition {
  fn size_of(&self) -> usize {
    match self {
      TypeDefinition::Primitive(primitive) => match primitive.native {
        NativeType::Void => panic!(),
        NativeType::U8 | NativeType::I8 => 1,
        NativeType::U16 | NativeType::I16 => 2,
        NativeType::U32 | NativeType::I32 | NativeType::F32 => 4,
        NativeType::U64
        | NativeType::I64
        | NativeType::USize
        | NativeType::ISize
        | NativeType::F64
        | NativeType::Pointer => 8,
      },
      TypeDefinition::Pointer(_) => 8,
      TypeDefinition::Buffer(buffer) => match buffer.r#type {
        BufferType::None | BufferType::U8 | BufferType::I8 => buffer.length,
        BufferType::U16 | BufferType::I16 => buffer.length * 2,
        BufferType::U32 | BufferType::I32 | BufferType::F32 => {
          buffer.length * 4
        }
        BufferType::U64
        | BufferType::I64
        | BufferType::USize
        | BufferType::ISize
        | BufferType::F64 => buffer.length * 8,
      },
      TypeDefinition::CString => 8,
      TypeDefinition::Struct(r#struct) => {
        let mut offset = 0;
        for (_, definition, _) in r#struct.fields() {
          offset += calculate_padding(offset, definition.align_of())
            + definition.size_of();
        }
        offset + calculate_padding(offset, self.align_of())
      }
    }
  }

  fn align_of(&self) -> usize {
    match self {
      TypeDefinition::Primitive(primitive) => match primitive.native {
        NativeType::Void => panic!(),
        NativeType::U8 | NativeType::I8 => 1,
        NativeType::U16 | NativeType::I16 => 2,
        NativeType::U32 | NativeType::I32 | NativeType::F32 => 4,
        NativeType::U64
        | NativeType::I64
        | NativeType::USize
        | NativeType::ISize
        | NativeType::F64
        | NativeType::Pointer => 8,
      },
      TypeDefinition::Pointer(_) => 8,
      TypeDefinition::Buffer(buffer) => match buffer.r#type {
        BufferType::None | BufferType::U8 | BufferType::I8 => 1,
        BufferType::U16 | BufferType::I16 => 2,
        BufferType::U32 | BufferType::I32 | BufferType::F32 => 4,
        BufferType::U64
        | BufferType::I64
        | BufferType::USize
        | BufferType::ISize
        | BufferType::F64 => 8,
      },
      TypeDefinition::CString => 8,
      TypeDefinition::Struct(r#struct) => r#struct
        .fields()
        .iter()
        .map(|(_, definition, _)| definition.align_of())
        .max()
        .unwrap_or(0),
    }
  }
}

impl From<TypeDefinition> for TypeDescriptor {
  fn from(definition: TypeDefinition) -> Self {
    match definition {
      TypeDefinition::Primitive(primitive) => TypeDescriptor::from(primitive),
      TypeDefinition::Pointer(pointer) => TypeDescriptor::from(pointer),
      TypeDefinition::Buffer(buffer) => TypeDescriptor::from(buffer),
      TypeDefinition::CString => TypeDescriptor::from(CString),
      TypeDefinition::Struct(r#struct) => TypeDescriptor::from(r#struct),
    }
  }
}

pub struct TypeDescriptor {
  pub native: NativeType,
  pub converter: TypeConverter,
}

impl TypeDescriptor {
  pub fn returns(&self) -> bool {
    !matches!(self.native, NativeType::Void)
  }
}

pub struct TypeConverter {
  pub globals: Vec<String>,
  pub typescript: String,
  pub into: String,
  pub from: String,
}

impl From<NativeType> for BufferType {
  fn from(native_type: NativeType) -> Self {
    match native_type {
      NativeType::U8 => BufferType::U8,
      NativeType::I8 => BufferType::I8,
      NativeType::U16 => BufferType::U16,
      NativeType::I16 => BufferType::I16,
      NativeType::U32 => BufferType::U32,
      NativeType::I32 => BufferType::I32,
      NativeType::U64 => BufferType::U64,
      NativeType::I64 => BufferType::I64,
      NativeType::USize => BufferType::USize,
      NativeType::ISize => BufferType::ISize,
      NativeType::F32 => BufferType::F32,
      NativeType::F64 => BufferType::F64,
      NativeType::Pointer => BufferType::U64,
      _ => BufferType::None,
    }
  }
}

impl From<BufferType> for String {
  fn from(buffer_type: BufferType) -> Self {
    match buffer_type {
      BufferType::None => "ArrayBuffer",
      BufferType::U8 => "Uint8Array",
      BufferType::I8 => "Int8Array",
      BufferType::U16 => "Uint16Array",
      BufferType::I16 => "Int16Array",
      BufferType::U32 => "Uint32Array",
      BufferType::I32 => "Int32Array",
      BufferType::U64 | BufferType::USize => "BigUint64Array",
      BufferType::I64 | BufferType::ISize => "BigInt64Array",
      BufferType::F32 => "Float32Array",
      BufferType::F64 => "Float64Array",
    }
    .to_string()
  }
}

impl From<NativeType> for String {
  fn from(native_type: NativeType) -> Self {
    match native_type {
      NativeType::Void => "void",
      NativeType::U8 => "u8",
      NativeType::I8 => "i8",
      NativeType::U16 => "u16",
      NativeType::I16 => "i16",
      NativeType::U32 => "u32",
      NativeType::I32 => "i32",
      NativeType::U64 => "u64",
      NativeType::I64 => "i64",
      NativeType::USize => "usize",
      NativeType::ISize => "isize",
      NativeType::F32 => "f32",
      NativeType::F64 => "f64",
      NativeType::Pointer => "pointer",
    }
    .to_string()
  }
}
