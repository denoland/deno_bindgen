use std::{io::Result, path::Path, process::Command};

#[derive(Default)]
pub struct Build {
  release: bool,
}

impl Build {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn release(mut self, release: bool) -> Self {
    self.release = release;
    self
  }

  pub fn build(self, path: &Path) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(path).arg("build").arg("--lib");

    if self.release {
      cmd.arg("--release");
    }

    let status = cmd.status()?;

    if status.success() {
      Ok(())
    } else {
      println!(
        "failed to execute `cargo`: exited with {}\n  full command: {:?}",
        status, cmd,
      );

      std::process::exit(1);
    }
  }
}
