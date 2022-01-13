use super::{
  NativeType, TypeConverter, TypeConverters, TypeDefinition, TypeDescriptor,
};

pub trait Sizeof {
  fn byte_size(&self) -> usize;
}

#[derive(Clone)]
pub struct StructLayout {
  pub size: usize,
  pub align: usize,
  pub fields: Vec<(String, TypeDefinition)>,
}

#[derive(Clone)]
pub struct Struct {
  pub name: String,
  pub layout: StructLayout,
}

impl Struct {
  pub fn new(name: &str, layout: StructLayout) -> Self {
    Self {
      name: name.to_string(),
      layout,
    }
  }
}

impl From<Struct> for TypeDescriptor {
  fn from(r#struct: Struct) -> Self {
    let mut typescript_properties: Vec<String> = Vec::new();
    let name = r#struct.name;
    let layout = r#struct.layout;

    for (property, definition) in layout.fields {
      let descriptor = TypeDescriptor::from(definition);
      typescript_properties.push(format!(
        "  {}: {};\n",
        property, descriptor.converters.typescript
      ));
    }

    TypeDescriptor {
      native: NativeType::Pointer,
      converters: TypeConverters {
        global: Some(format!(
          "export interface {} {{\n{}}}",
          name,
          typescript_properties.join("")
        )),
        typescript: name,
        into: todo!(),
        from: todo!(),
      },
    }
  }
}
