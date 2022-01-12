use std::fmt::write;
use std::fmt::Arguments;
use std::fmt::Result;
use std::fmt::Write;

pub struct Source(String);

impl Source {
  pub fn new() -> Self {
    Self(String::new())
  }

  pub fn read(&self) -> String {
    self.0.clone()
  }
}

impl Write for Source {
  fn write_str(&mut self, s: &str) -> Result {
    Write::write_str(&mut self.0, s)
  }

  fn write_char(&mut self, c: char) -> Result {
    self.write_str(c.encode_utf8(&mut [0; 4]))
  }

  fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> Result {
    write(&mut self, args)
  }
}
