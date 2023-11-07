use std::path::PathBuf;

use structopt::StructOpt;

mod cargo;
mod dlfcn;

#[derive(Debug, StructOpt)]
#[structopt(name = "deno_bindgen_cli", about = "A CLI for deno_bindgen")]
struct Opt {
  #[structopt(short, long)]
  /// Build in release mode
  release: bool,

  #[structopt(short, long)]
  out: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
  let opt = Opt::from_args();

  let cwd = std::env::current_dir().unwrap();
  cargo::Build::new().release(opt.release).build(&cwd)?;

  unsafe {
    dlfcn::load_and_init(
      &cwd.join("target/debug/libdeno_bindgen_test.dylib"),
      opt.out,
    )?
  };
  Ok(())
}
