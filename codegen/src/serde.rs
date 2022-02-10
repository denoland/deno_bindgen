use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
  function::{Function, FunctionParameters},
  library::Library,
  loader::{deno::DenoLoader, plug::PlugLoader},
  types::TypeDefinition,
};

#[derive(Serialize, Deserialize)]
pub struct SerdeLibrary {
  pub variable: Option<String>,
  pub loader: SerdeLoader,
  pub types: HashMap<String, TypeDefinition>,
  pub functions: HashMap<String, SerdeFunction>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SerdeLoader {
  Deno(DenoLoader),
  Plug(PlugLoader),
}

#[derive(Serialize, Deserialize)]
pub struct SerdeFunction {
  name: Option<String>,
  docs: Option<String>,
  parameters: FunctionParameters,
  result: String,
  nonblocking: bool,
  export: bool,
}

#[cfg(feature = "serde")]
impl From<SerdeLibrary> for Library {
  fn from(serde_library: SerdeLibrary) -> Self {
    let mut library = Library::new(
      serde_library.variable.as_deref(),
      match serde_library.loader {
        SerdeLoader::Deno(loader) => Box::new(loader),
        SerdeLoader::Plug(loader) => Box::new(loader),
      },
    );

    for (name, definition) in serde_library.types {
      library.register_type(&name, definition)
    }

    for (symbol, function) in serde_library.functions {
      library.append(Box::new(Function::new(
        &symbol,
        function.name.as_deref(),
        function.docs.as_deref(),
        function.parameters,
        &function.result,
        function.nonblocking,
        function.export,
      )));
    }

    library
  }
}
