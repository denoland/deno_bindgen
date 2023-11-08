use std::path::{Path, PathBuf};

use dlopen2::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct Api {
  init_deno_bindgen: unsafe fn(opt: deno_bindgen_ir::codegen::Options),
}

pub unsafe fn load_and_init(
  path: &Path,
  out: Option<PathBuf>,
) -> std::io::Result<()> {
  let cont: Container<Api> = Container::load(path).map_err(|e| {
    std::io::Error::new(
      std::io::ErrorKind::Other,
      format!("failed to load library: {}", e),
    )
  })?;

  cont.init_deno_bindgen(deno_bindgen_ir::codegen::Options {
    target: deno_bindgen_ir::codegen::Target::Deno,
    out,
  });

  Ok(())
}