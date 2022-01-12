use std::collections::HashMap;
use std::fmt::Write;

use crate::error::{unknown_type, AnyError};
use crate::source::Source;
use crate::types::{TypeDefinition, TypeDescriptor};

pub struct Library {
  pub variable: String,
  types: HashMap<String, TypeDescriptor>,
  loader: Box<dyn LibraryElement>,
  elements: Vec<Box<dyn LibraryElement>>,
}

impl Library {
  pub fn new(variable: Option<&str>, loader: Box<dyn LibraryElement>) -> Self {
    Self {
      variable: variable.unwrap_or("library").to_string(),
      types: HashMap::new(),
      loader,
      elements: Vec::new(),
    }
  }

  pub fn register_type(&mut self, name: &str, definiton: TypeDefinition) {
    self.types.insert(name.to_string(), definiton.into());
  }

  pub fn lookup_type(&self, name: &str) -> Result<&TypeDescriptor, AnyError> {
    self.types.get(name).ok_or(unknown_type(name))
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

    self.loader.generate(self, &mut source)?;

    for (_, descriptor) in &self.types {
      if let Some(global) = &descriptor.converters.into.global {
        global.generate(self, &mut source)?;
      }

      if let Some(global) = &descriptor.converters.from.global {
        global.generate(self, &mut source)?;
      }
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
  fn symbol(&self, library: &Library) -> Result<Option<String>, AnyError> {
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
  use crate::types::BufferType;
  use crate::types::NativeType;
  use crate::types::TypeDefinition;

  #[test]
  fn testing() {
    let mut library = Library::new(
      None,
      Box::new(PlugLoader::new(
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

    library
      .register_type("usize", TypeDefinition::Primitive(NativeType::USize));
    library.register_type("cstring", TypeDefinition::CString);
    library.register_type(
      "[usize]",
      TypeDefinition::Buffer(BufferType::USize, None),
    );

    library.append(Box::new(Function::new(
      "test",
      None,
      None,
      FunctionParameters::Unnamed(vec![
        "usize".to_string(),
        "cstring".to_string(),
      ]),
      "[usize]",
      false,
    )));

    let source = library.generate().unwrap();
    println!("{}", source.read());
  }
}
