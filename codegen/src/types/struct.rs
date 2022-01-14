use inflector::Inflector;

use super::TypeDescriptor;
use super::TypeDefinition;
use super::TypeConverter;
use super::NativeType;

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
    let type_name = r#struct.name.to_pascal_case();
    let variable_name = r#struct.name.to_snake_case();
    let layout = r#struct.layout;

    let views_required: Vec<String> = Vec::new();
    let into_function_name = format!("__{}_into", variable_name);
    let from_function_name = format!("__{}_from", variable_name);
    let mut into_function_body = String::new();
    let mut from_function_body = String::new();

    for (property, definition) in layout.fields {
      let descriptor = TypeDescriptor::from(definition);
      typescript_properties.push(format!(
        "  {}: {};\n",
        property, descriptor.converter.typescript
      ));

//      descriptor.converter.from

      into_function_body += "";
      from_function_body += "";
    }

    let typescript_interface = format!(
      "export interface {} {{\n{}}}",
      type_name,
      typescript_properties.join("")
    );
    let into_function = format!(
      "function {}({}: {}): Deno.UnsafePointer {{\n{}\n}}",
      into_function_name, variable_name, type_name, into_function_body
    );
    let from_function = format!(
      "function {}({}: Deno.UnsafePointer): {} {{\n{}\n}}",
      from_function_name, variable_name, type_name, from_function_body
    );

    TypeDescriptor {
      native: NativeType::Pointer,
      converter: TypeConverter {
        global: Some(format!(
          "{}\n{}\n{}\n",
          typescript_interface, into_function, from_function
        )),
        typescript: type_name,
        into: format!("{}({{}})", into_function_name),
        from: format!("{}({{}})", from_function_name),
      },
    }
  }
}
