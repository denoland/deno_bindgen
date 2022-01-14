use std::collections::HashMap;
use std::fmt::Write;

use crate::error::AnyError;
use crate::library::Library;
use crate::library::LibraryElement;
use crate::source::Source;
use crate::types::TypeDescriptor;

pub enum FunctionParameters {
  Named(HashMap<String, String>),
  Unnamed(Vec<String>),
}

pub struct Function {
  symbol: String,
  name: String,
  docs: Option<String>,
  parameters: HashMap<String, String>,
  result: String,
  nonblocking: bool,
  export: bool
}

impl Function {
  pub fn new(
    symbol: &str,
    name: Option<&str>,
    docs: Option<&str>,
    parameters: FunctionParameters,
    result: &str,
    nonblocking: bool,
    export: bool
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
    };

    Self {
      symbol,
      name,
      docs: docs.map(String::from),
      parameters,
      result: result.to_string(),
      nonblocking,
      export
    }
  }
}

impl LibraryElement for Function {
  fn symbol(&self, library: &Library) -> Result<Option<String>, AnyError> {
    let parameters = self
      .parameters
      .iter()
      .map(|(_, parameter)| -> Result<String, AnyError> {
        let parameter = library.lookup_type(parameter)?;
        let native = String::from(parameter.native);
        Ok(format!("\"{}\"", native))
      })
      .collect::<Result<Vec<String>, AnyError>>()?
      .join(", ");

    let result = String::from(library.lookup_type(&self.result)?.native);

    Ok(Some(format!(
      "{}: {{ parameters: [{}], result: \"{}\", nonblocking: {} }}",
      self.symbol, parameters, result, self.nonblocking
    )))
  }

  fn generate(
    &self,
    library: &Library,
    source: &mut Source,
  ) -> Result<(), AnyError> {
    let result = library.lookup_type(&self.result)?;
    let parameters = self
      .parameters
      .iter()
      .map(|(name, parameter)| Ok((name, library.lookup_type(parameter)?)))
      .collect::<Result<Vec<(&String, &TypeDescriptor)>, AnyError>>()?;

    if let Some(docs) = &self.docs {
      writeln!(source, "{}", docs)?;
    }

    if self.export {
      write!(source, "export ")?;
    }

    if self.nonblocking {
      write!(source, "async ")?;
    }

    write!(source, "function {}(", self.name)?;

    write!(
      source,
      "{}",
      parameters
        .iter()
        .map(|(name, parameter)| format!(
          "{}: {}",
          name, parameter.converter.typescript
        ))
        .collect::<Vec<String>>()
        .join(", ")
    )?;

    if self.nonblocking {
      writeln!(source, "): Promise<{}> {{", result.converter.typescript)?;
    } else {
      writeln!(source, "): {} {{", result.converter.typescript)?;
    }

    writeln!(
      source,
      "{}{};",
      if result.returns() { "return " } else { "" },
      result.converter.from.replace(
        "{}",
        &format!(
          "{}.symbols.{}({})",
          library.variable,
          self.symbol,
          parameters
            .iter()
            .map(|(name, parameter)| parameter
              .converter
              .into
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
