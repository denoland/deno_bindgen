use std::fmt::Write;

use crate::error::AnyError;
use crate::library::Library;
use crate::library::LibraryElement;
use crate::source::Source;

pub struct DenoLoader {
  pub filename: String,
}

impl DenoLoader {
  pub fn new(filename: &str) -> Self {
    Self {
      filename: filename.to_string(),
    }
  }
}

impl LibraryElement for DenoLoader {
  fn generate(
    &self,
    library: &Library,
    source: &mut Source,
  ) -> Result<(), AnyError> {
    writeln!(
      source,
      "const {} = await Plug.prepare(\"{}\", {});",
      library.variable,
      self.filename,
      library.symbols()?
    )?;

    Ok(())
  }
}
