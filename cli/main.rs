use std::{
  fs::{File, OpenOptions},
  io::{self, Read, Write},
};

use clap::Parser;
use deno_bindgen_codegen::{library::Library, serde::SerdeLibrary};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  input: String,
  output: Option<String>,
}

fn main() {
  let args = Args::parse();
  let mut json = String::new();

  File::open(args.input)
    .expect("Could not open input file")
    .read_to_string(&mut json)
    .expect("Could not read input file");

  let mut library: Library = serde_json::from_str::<SerdeLibrary>(&json)
    .expect("Could not deserialize json")
    .into();
  let source = library.generate().expect("Could not generate source");

  if let Some(output) = args.output {
    OpenOptions::new()
      .write(true)
      .create(true)
      .open(&output)
      .expect("Could not open output file")
      .write(source.read().as_bytes())
      .expect("Could not write output file");
  } else {
    println!("{}", source.read());
  }
}
