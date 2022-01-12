use anyhow::anyhow;
use anyhow::Error;

pub type AnyError = anyhow::Error;

pub fn unknown_type(name: &str) -> Error {
  anyhow!("Unknown type: {}", name)
}
