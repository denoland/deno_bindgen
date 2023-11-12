use std::path::PathBuf;

use crate::inventory::Inventory;

mod deno;

pub struct Options {
  pub target: Target,
  pub out: Option<PathBuf>,
  pub local_dylib_path: PathBuf,
  pub lazy_init: bool,
}

pub enum Target {
  Deno,
}

pub trait Generator {
  fn generate<W: std::io::Write>(&mut self, writer: W) -> std::io::Result<()>;
}

pub fn generate(
  symbols: &'static [Inventory],
  opt: Options,
) -> std::io::Result<()> {
  let mut codegen = match opt.target {
    Target::Deno => {
      deno::Codegen::new(symbols, &opt.local_dylib_path, opt.lazy_init)
    }
  };

  if let Some(out) = opt.out {
    let mut writer = std::fs::File::create(out)?;
    codegen.generate(&mut writer)?;
    return Ok(());
  }

  let writer = std::io::stdout();
  codegen.generate(writer)
}
