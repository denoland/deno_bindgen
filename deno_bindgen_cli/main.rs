use std::path::PathBuf;

use cargo::Artifact;
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

  #[structopt(short, long)]
  lazy_init: bool,
}

fn main() -> std::io::Result<()> {
  let opt = Opt::from_args();

  let cwd = std::env::current_dir().unwrap();
  let Artifact { path, .. } =
    cargo::Build::new().release(opt.release).build(&cwd)?;

  let name = cargo::metadata()?;
  println!("Initializing {name}");

  unsafe {
    dlfcn::load_and_init(&PathBuf::from(path), opt.out, opt.lazy_init)?
  };

  println!("Ready {name}");
  Ok(())
}
