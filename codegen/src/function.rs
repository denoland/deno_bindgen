use std::collections::HashMap;
use std::fmt::Write;

use crate::library::Library;
use crate::library::LibraryElement;
use crate::source::Source;
use crate::types::TypeDefiniton;
use crate::types::TypeDescriptor;
use crate::AnyError;

pub enum FunctionParameters {
  Named(HashMap<String, TypeDefiniton>),
  Unnamed(Vec<TypeDefiniton>),
}

pub struct Function {
  symbol: String,
  name: String,
  docs: Option<String>,
  parameters: HashMap<String, TypeDescriptor>,
  result: TypeDescriptor,
  nonblocking: bool,
}

impl Function {
  pub fn new(
    symbol: &str,
    name: Option<&str>,
    docs: Option<&str>,
    parameters: FunctionParameters,
    result: TypeDefiniton,
    nonblocking: bool,
  ) -> Self {
    let name = name.unwrap_or(symbol).to_string();
    let symbol = symbol.to_string();
    let parameters = match parameters {
      FunctionParameters::Named(parameters) => parameters,
      FunctionParameters::Unnamed(parameters) => {
        let mut map = HashMap::new();
        for (index, parameter) in parameters.into_iter().enumerate() {
          map.insert(format!("parameter{}", index), parameter);
        }
        map
      }
    }
    .into_iter()
    .map(|(name, parameter)| (name, parameter.into()))
    .collect();
    let result = result.into();

    Self {
      symbol,
      name,
      docs: docs.map(String::from),
      parameters,
      result,
      nonblocking,
    }
  }
}

impl LibraryElement for Function {
  fn symbol(&self) -> Option<String> {
    Some(format!(
      "{}: {{ parameters: [{}], result: \"{}\", nonblocking: {} }}",
      self.symbol,
      self
        .parameters
        .iter()
        .map(|(name, parameter)| format!(
          "{}: \"{}\"",
          name,
          String::from(parameter.native.clone())
        ))
        .collect::<Vec<String>>()
        .join(", "),
      String::from(self.result.native),
      self.nonblocking
    ))
  }

  fn generate(
    &self,
    library: &Library,
    source: &mut Source,
  ) -> Result<(), AnyError> {
    if let Some(docs) = &self.docs {
      writeln!(source, "{}", docs)?;
    }

    write!(source, "export function ")?;

    if self.nonblocking {
      write!(source, "async ")?;
    }

    write!(source, "{}(", self.name)?;

    write!(
      source,
      "{}",
      self
        .parameters
        .iter()
        .map(|(name, parameter)| format!(
          "{}: {}",
          name, parameter.converters.into.typescript
        ))
        .collect::<Vec<String>>()
        .join(", ")
    )?;

    if self.nonblocking {
      writeln!(
        source,
        "): Promise<{}> {{",
        self.result.converters.from.typescript
      )?;
    } else {
      writeln!(source, "): {} {{", self.result.converters.from.typescript)?;
    }

    if self.result.returns() {
      write!(source, "return ")?;
    }

    writeln!(
      source,
      "{};",
      self.result.converters.from.inline.replace(
        "{}",
        &format!(
          "{}.symbols.{}({})",
          library.variable,
          self.symbol,
          self
            .parameters
            .iter()
            .map(|(name, parameter)| parameter
              .converters
              .into
              .inline
              .replace("{}", name))
            .collect::<Vec<String>>()
            .join(", ")
        )
      )
    )?;

    writeln!(source, "}}")?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::library::Library;
  use crate::loader::plug::PlugLoader;
  use crate::loader::plug::PlugLoaderOptions;
  use crate::loader::plug::PlugLoaderSingleOptions;
  use crate::types::BufferType;
  use crate::types::TypeDefiniton;

  use super::{Function, FunctionParameters};

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

    library.append(Box::new(Function::new(
      "test",
      None,
      None,
      FunctionParameters::Unnamed(vec![
        TypeDefiniton::Buffer(BufferType::USize, None),
        TypeDefiniton::CString,
      ]),
      TypeDefiniton::Buffer(BufferType::USize, None),
      false,
    )));

    let source = library.generate().unwrap();
    println!("{}", source.read());
  }
}
