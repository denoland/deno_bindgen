use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

pub struct Artifact {
  pub path: PathBuf,
  pub manifest_path: PathBuf,
}

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

  pub fn build(self, path: &Path) -> Result<Artifact> {
    let mut cmd = Command::new("cargo");
    cmd
      .current_dir(path)
      .arg("build")
      .arg("--lib")
      .arg("--message-format=json")
      .stdout(Stdio::piped());

    if self.release {
      cmd.arg("--release");
    }

    let status = cmd.status()?;
    let output = cmd.output()?;
    if status.success() {
      let reader = std::io::BufReader::new(output.stdout.as_slice());
      let mut artifacts = vec![];
      for message in cargo_metadata::Message::parse_stream(reader) {
        match message.unwrap() {
          cargo_metadata::Message::CompilerArtifact(artifact) => {
            if artifact.target.kind.contains(&"cdylib".to_string()) {
              artifacts.push(Artifact {
                path: PathBuf::from(artifact.filenames[0].to_string()),
                manifest_path: PathBuf::from(
                  artifact.manifest_path.to_string(),
                ),
              });
            }
          }
          _ => {}
        }
      }

      // TODO: Fix. Not an ideal way to get the artifact of the desired crate, but it
      // works for most case.
      if let Some(artifact) = artifacts.pop() {
        return Ok(artifact);
      }

      Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "failed to parse cargo output",
      ))?
    } else {
      println!(
        "failed to execute `cargo`: exited with {}\n  full command: {:?}",
        status, cmd,
      );

      std::process::exit(1);
    }
  }
}

pub fn metadata() -> Result<String> {
  let metadata = cargo_metadata::MetadataCommand::new()
    .exec()
    .map_err(|e| {
      println!("failed to execute `cargo metadata`: {}", e);
      std::process::exit(1);
    })
    .unwrap();

  Ok(metadata.root_package().unwrap().name.clone())
}
