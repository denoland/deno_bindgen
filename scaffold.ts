import { basename } from "https://deno.land/std@0.145.0/path/mod.ts";

const libPath = Deno.args[0];

// 1- Create cargo fixture
Deno.mkdirSync(libPath + "/src", { recursive: true });

// 2- Create lib.rs
Deno.writeTextFileSync(
  libPath + "/src/lib.rs",
  `\
// add.rs
use deno_bindgen::deno_bindgen;

#[deno_bindgen]
pub struct Input {
  a: i32,
  b: i32,
}

#[deno_bindgen]
fn add(input: Input) -> i32 {
  input.a + input.b
}`,
);

// 3- Create Cargo.toml
Deno.writeTextFileSync(
  libPath + "/Cargo.toml",
  `\
[package]
name = "${basename(libPath)}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
deno_bindgen = "0.7.0"
serde = { version = "1", features = ["derive"] }

[lib]
crate-type = ["cdylib"]`,
);

// 4- Create mod.ts
Deno.writeTextFileSync(
  libPath + "/mod.ts",
  `\
// add.ts
import { add } from "./bindings/bindings.ts";

console.log(
  add({ a: 1, b: 2 }),
);`,
);

// 5- Create github action
// TODO

console.log("1- cd ", libPath);
console.log("2- Run deno_bindgen");
console.log("3- Run deno run --unstable lib.ts");
