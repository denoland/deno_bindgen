use std::fmt::Write;
use std::iter::FromIterator;

use indexmap::{IndexMap, IndexSet};

use crate::error::{unknown_type, AnyError};
use crate::source::Source;
use crate::types::{TypeDefinition, TypeDescriptor};

pub struct Library {
  pub variable: String,
  types: IndexMap<String, TypeDescriptor>,
  loader: Box<dyn LibraryElement>,
  elements: Vec<Box<dyn LibraryElement>>,
}

impl Library {
  pub fn new(variable: Option<&str>, loader: Box<dyn LibraryElement>) -> Self {
    Self {
      variable: variable.unwrap_or("library").to_string(),
      types: IndexMap::new(),
      loader,
      elements: Vec::new(),
    }
  }

  pub fn register_type(&mut self, name: &str, definiton: TypeDefinition) {
    self.types.insert(name.to_string(), definiton.into());
  }

  pub fn lookup_type(&self, name: &str) -> Result<&TypeDescriptor, AnyError> {
    self.types.get(name).ok_or_else(|| unknown_type(name))
  }

  pub fn prepend(&mut self, element: Box<dyn LibraryElement>) {
    self.elements.insert(0, element)
  }

  pub fn append(&mut self, element: Box<dyn LibraryElement>) {
    self.elements.push(element)
  }

  pub fn symbols(&self) -> Result<String, AnyError> {
    let symbols = self
      .elements
      .iter()
      .filter_map(|element| element.symbol(self).transpose())
      .collect::<Result<Vec<String>, AnyError>>()?
      .join(", ");
    Ok(format!("{{ {} }}", symbols))
  }

  pub fn generate(&mut self) -> Result<Source, AnyError> {
    let mut source = Source::new();
    let globals: IndexSet<String> = IndexSet::from_iter(
      self
        .types
        .values()
        .map(|descriptor| descriptor.converter.globals.clone())
        .flatten(),
    );

    self.loader.generate(self, &mut source)?;

    for global in globals {
      global.generate(self, &mut source)?;
    }

    for element in &self.elements {
      element.generate(self, &mut source)?;
    }

    Ok(source)
  }
}

pub trait LibraryElement {
  fn generate(
    &self,
    library: &Library,
    source: &mut Source,
  ) -> Result<(), AnyError>;
  fn symbol(&self, _library: &Library) -> Result<Option<String>, AnyError> {
    Ok(None)
  }
}

impl LibraryElement for String {
  fn generate(&self, _: &Library, source: &mut Source) -> Result<(), AnyError> {
    source.write_str(self)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::function::Function;
  use crate::function::FunctionParameters;
  use crate::library::Library;
  use crate::loader::plug::PlugLoader;
  use crate::loader::plug::PlugLoaderOptions;
  use crate::loader::plug::PlugLoaderSingleOptions;
  use crate::types::buffer::Buffer;
  use crate::types::pointer::Pointer;
  use crate::types::primitive::Primitive;
  use crate::types::r#struct::Struct;
  use crate::types::tuple::Tuple;
  use crate::types::BufferType;
  use crate::types::NativeType;
  use crate::types::TypeDefinition;

  #[test]
  fn testing() {
    let mut library = Library::new(
      None,
      Box::new(PlugLoader::new(
        true,
        None,
        PlugLoaderOptions::Single(PlugLoaderSingleOptions {
          name: "test".to_string(),
          url: "abcdef".to_string(),
          policy: None,
          cache: None,
          log: None,
        }),
      )),
    );

    library.register_type(
      "usize",
      TypeDefinition::Primitive(Primitive::new(NativeType::USize)),
    );
    library.register_type("cstring", TypeDefinition::CString);
    library.register_type(
      "ExampleStruct",
      TypeDefinition::Struct(Struct::new(
        Some("ExampleStruct"),
        true,
        vec![
          (
            "a".to_string(),
            TypeDefinition::Pointer(Pointer::new(Box::new(
              TypeDefinition::Primitive(Primitive::new(NativeType::U16)),
            ))),
          ),
          ("b".to_string(), TypeDefinition::CString),
          (
            "c".to_string(),
            TypeDefinition::Primitive(Primitive::new(NativeType::Pointer)),
          ),
          (
            "d".to_string(),
            TypeDefinition::Primitive(Primitive::new(NativeType::I8)),
          ),
          (
            "f".to_string(),
            TypeDefinition::Buffer(Buffer::new(BufferType::I64, 13)),
          ),
          (
            "g".to_string(),
            TypeDefinition::Struct(Struct::new(
              Some("ExampleInnerStruct"),
              false,
              vec![
                (
                  "inner_a".to_string(),
                  TypeDefinition::Primitive(Primitive::new(NativeType::U32)),
                ),
                (
                  "inner_b".to_string(),
                  TypeDefinition::Primitive(Primitive::new(
                    NativeType::Pointer,
                  )),
                ),
              ],
            )),
          ),
        ],
      )),
    );

    library.register_type(
      "TestTuple",
      TypeDefinition::Tuple(Tuple::new(
        Some("TestTuple"),
        true,
        vec![
          TypeDefinition::Pointer(Pointer::new(Box::new(
            TypeDefinition::Primitive(Primitive::new(NativeType::U16)),
          ))),
          TypeDefinition::CString,
          TypeDefinition::Primitive(Primitive::new(NativeType::Pointer)),
        ],
      )),
    );

    library.append(Box::new(Function::new(
      "test",
      None,
      None,
      FunctionParameters::Unnamed(vec!["usize".to_string()]),
      "ExampleStruct",
      false,
      true,
    )));

    let source = library.generate().unwrap();
    println!("{}", source.read());
  }
}
