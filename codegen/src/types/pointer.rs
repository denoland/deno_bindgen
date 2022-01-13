use super::{
  BufferType, NativeType, TypeConverter, TypeConverters, TypeDefinition,
  TypeDescriptor,
};

#[derive(Clone)]
pub struct Pointer {
  pub target: Box<TypeDefinition>,
}

impl Pointer {
  pub fn new(target: Box<TypeDefinition>) -> Self {
    Self { target }
  }
}

impl From<Pointer> for TypeDescriptor {
  fn from(pointer: Pointer) -> Self {
    let target = pointer.target.as_ref();
    let target_descriptor: TypeDescriptor = target.clone().into();
    let converters = if let TypeDefinition::Primitive(primitive) = target {
      let native = primitive.native;

      if let NativeType::Void | NativeType::Pointer = native {
        TypeConverters {
          global: target_descriptor.converters.global,
          typescript: target_descriptor.converters.typescript,
          into: TypeConverter {
            local: target_descriptor.converters.into.local,
            inline: format!(
              "Deno.UnsafePointer.of(new BigUint64Array([{}.value]))",
              target_descriptor.converters.into.inline
            ),
          },
          from: TypeConverter {
            local: target_descriptor.converters.from.local,
            inline: "".to_string(),
          },
        }
      } else {
        let buffer_type: BufferType = native.into();
        let constructor: String = buffer_type.into();

        TypeConverters {
          global: target_descriptor.converters.global,
          typescript: target_descriptor.converters.typescript,
          into: TypeConverter {
            local: target_descriptor.converters.into.local,
            inline: format!(
              "Deno.UnsafePointer.of(new {}([{}.value]))",
              constructor, target_descriptor.converters.into.inline
            ),
          },
          from: TypeConverter {
            local: None,
            inline: "".to_string(),
          },
        }
      }
    } else {
      TypeConverters {
        global: target_descriptor.converters.global,
        typescript: target_descriptor.converters.typescript,
        into: TypeConverter {
          local: target_descriptor.converters.into.local,
          inline: format!(
            "Deno.UnsafePointer.of(new BigUint64Array([{}.value]))",
            target_descriptor.converters.into.inline
          ),
        },
        from: TypeConverter {
          local: target_descriptor.converters.from.local,
          inline: format!(
            "new Deno.UnsafePointer(new Deno.UnsafePointerView({}).getBigUint64())",
            target_descriptor.converters.from.inline
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
