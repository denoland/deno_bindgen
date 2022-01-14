use super::BufferType;
use super::NativeType;
use super::TypeConverter;
use super::TypeDefinition;
use super::TypeDescriptor;

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
    let converter = if let TypeDefinition::Primitive(primitive) = target {
      let native = primitive.native;

      if let NativeType::Void | NativeType::Pointer = native {
        TypeConverter {
          global: target_descriptor.converter.global,
          typescript: target_descriptor.converter.typescript,
          into: format!(
            "Deno.UnsafePointer.of(new BigUint64Array([{}.value]))",
            target_descriptor.converter.into
          ),
          from: "".to_string(),
        }
      } else {
        let buffer_type: BufferType = native.into();
        let constructor: String = buffer_type.into();

        TypeConverter {
          global: target_descriptor.converter.global,
          typescript: target_descriptor.converter.typescript,
          into: format!(
            "Deno.UnsafePointer.of(new {}([{}.value]))",
            constructor, target_descriptor.converter.into
          ),
          from: "".to_string(),
        }
      }
    } else {
      TypeConverter {
        global: target_descriptor.converter.global,
        typescript: target_descriptor.converter.typescript,
        into: format!(
          "Deno.UnsafePointer.of(new BigUint64Array([{}.value]))",
          target_descriptor.converter.into
        ),
        from: format!(
          "new Deno.UnsafePointer(new Deno.UnsafePointerView({}).getBigUint64())",
          target_descriptor.converter.from
        ),
      }
    };

    TypeDescriptor {
      native: NativeType::Pointer,
      converter,
    }
  }
}
