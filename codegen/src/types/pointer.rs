use super::BufferType;
use super::NativeType;
use super::TypeConverter;
use super::TypeDefinition;
use super::TypeDescriptor;

#[derive(Clone, Hash)]
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

      if let NativeType::Pointer = native {
        TypeConverter {
          globals: target_descriptor.converter.globals,
          typescript: target_descriptor.converter.typescript,
          into: format!(
            "Deno.UnsafePointer.of(new BigUint64Array([{}.value]))",
            target_descriptor.converter.into
          ),
          from: "{}".to_string(),
        }
      } else {
        let buffer_type: BufferType = native.into();
        let constructor = buffer_type.typed_array();
        let getter = buffer_type.pointer_view_getter();

        TypeConverter {
          globals: target_descriptor.converter.globals,
          typescript: target_descriptor.converter.typescript,
          into: format!(
            "Deno.UnsafePointer.of(new {}([{}]))",
            constructor, target_descriptor.converter.into
          ),
          from: if let BufferType::None = buffer_type {
            "/* ? */".to_string()
          } else {
            format!("new Deno.UnsafePointerView({{}}).{}(0)", getter)
          },
        }
      }
    } else {
      TypeConverter {
        globals: target_descriptor.converter.globals,
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
