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
  Primitive(NativeType),
  Pointer(Box<TypeDefinition>),
  Buffer(BufferType, Option<usize>),
  CString,
}

pub struct TypeDescriptor {
  pub definition: TypeDefinition,
  pub native: NativeType,
  pub converters: TypeConverters,
}

impl TypeDescriptor {
  pub fn returns(&self) -> bool {
    if let NativeType::Void = self.native {
      false
    } else {
      true
    }
  }
}

pub struct TypeConverter {
  pub typescript: String,
  pub global: Option<String>,
  pub local: Option<String>,
  pub inline: String,
}

pub struct TypeConverters {
  pub into: TypeConverter,
  pub from: TypeConverter,
}

impl From<TypeDefinition> for TypeDescriptor {
  fn from(definition: TypeDefinition) -> Self {
    match definition {
      TypeDefinition::Primitive(native) => {
        let typescript = match native {
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
          definition,
          native,
          converters,
        }
      }
      TypeDefinition::Pointer(ref target) => {
        let target = target.as_ref();
        let target_descriptor: TypeDescriptor = target.clone().into();
        let converters = match target {
          TypeDefinition::Primitive(native) => {
            if let NativeType::Void | NativeType::Pointer = native {
              TypeConverters {
                into: TypeConverter {
                  typescript: target_descriptor.converters.into.typescript,
                  global: target_descriptor.converters.into.global,
                  local: target_descriptor.converters.into.local,
                  inline: format!("Deno.UnsafePointer.of(new BigUint64Array([{}.value]))", target_descriptor.converters.into.inline),
                },
                from: TypeConverter {
                  typescript: target_descriptor.converters.from.typescript,
                  global: target_descriptor.converters.from.global,
                  local: target_descriptor.converters.from.local,
                  inline: "".to_string(),
                },
              }
            } else {
              let buffer_type: BufferType = (*native).into();
              let constructor: String = buffer_type.into();

              TypeConverters {
                into: TypeConverter {
                  typescript: target_descriptor.converters.into.typescript,
                  global: target_descriptor.converters.into.global,
                  local: target_descriptor.converters.into.local,
                  inline: format!(
                    "Deno.UnsafePointer.of(new {}([{}.value]))",
                    constructor,
                    target_descriptor.converters.into.inline
                  ),
                },
                from: TypeConverter {
                  typescript: target_descriptor.converters.from.typescript,
                  global: None,
                  local: None,
                  inline: "".to_string(),
                },
              }
            }
          }
          TypeDefinition::Pointer(_)
          | TypeDefinition::Buffer(_, _)
          | TypeDefinition::CString => TypeConverters {
            into: TypeConverter {
              typescript: target_descriptor.converters.into.typescript,
              global: target_descriptor.converters.into.global,
              local: target_descriptor.converters.into.local,
              inline: format!(
                "Deno.UnsafePointer.of(new BigUint64Array([{}.value]))",
                target_descriptor.converters.into.inline
              ),
            },
            from: TypeConverter {
              typescript: target_descriptor.converters.from.typescript,
              global: target_descriptor.converters.from.global,
              local: target_descriptor.converters.from.local,
              inline: format!(
                "new Deno.UnsafePointer(new Deno.UnsafePointerView({}).getBigUint64())",
                target_descriptor.converters.from.inline
              ),
            },
          },
        };

        TypeDescriptor {
          definition,
          native: NativeType::Pointer,
          converters,
        }
      }
      TypeDefinition::Buffer(buffer_type, length) => {
        if let Some(length) = length {
          let converters = if let BufferType::None = buffer_type {
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
            let constructor: String = buffer_type.into();
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
            definition,
            native: NativeType::Pointer,
            converters,
          }
        } else {
          let constructor: String = buffer_type.into();
          TypeDescriptor {
            definition,
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
      TypeDefinition::CString => TypeDescriptor {
        definition,
        native: NativeType::Pointer,
        converters: TypeConverters {
          into: TypeConverter {
            typescript: "string".to_string(),
            global: Some(
              "const __encoder = new TextEncoder();\
              function __cstring_into(cstring: string): Deno.UnsafePointer {\
                const buffer = new Uint8Array(cstring.length + 1);\
                __encoder.encodeInto(cstring, buffer);\
                return Deno.UnsafePointer.of(buffer);\
              }\n"
              .to_string(),
            ),
            local: None,
            inline: "__cstring_into({})".to_string(),
          },
          from: TypeConverter {
            typescript: "string".to_string(),
            global: None,
            local: None,
            inline: "new Deno.UnsafePointerView({}).getCString()".to_string(),
          },
        },
      },
    }
  }
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
