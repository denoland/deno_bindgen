use std::fmt::Write;

use crate::source::Source;
use crate::AnyError;

pub struct Library {
  pub variable: String,
  loader: Box<dyn LibraryElement>,
  elements: Vec<Box<dyn LibraryElement>>,
}

impl Library {
  pub fn new(variable: Option<&str>, loader: Box<dyn LibraryElement>) -> Self {
    Self {
      variable: variable.unwrap_or("library").to_string(),
      loader,
      elements: Vec::new(),
    }
  }

  pub fn prepend(&mut self, element: Box<dyn LibraryElement>) {
    self.elements.insert(0, element)
  }

  pub fn append(&mut self, element: Box<dyn LibraryElement>) {
    self.elements.push(element)
  }

  pub fn symbols(&self) -> String {
    format!(
      "{{ {} }}",
      self
        .elements
        .iter()
        .filter_map(|element| element.symbol())
        .collect::<Vec<String>>()
        .join(", ")
    )
  }

  pub fn generate(&mut self) -> Result<Source, AnyError> {
    let mut source = Source::new();

    self.loader.generate(self, &mut source)?;

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
  fn symbol(&self) -> Option<String> {
    None
  }
}

impl LibraryElement for String {
  fn generate(&self, _: &Library, source: &mut Source) -> Result<(), AnyError> {
    source.write_str(self)?;
    Ok(())
  }
}
