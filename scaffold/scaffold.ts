import { basename } from "https://deno.land/std@0.145.0/path/mod.ts";

const libPath = Deno.args[0];
const libName = basename(libPath);

// 1- Create cargo fixture
try {
  Deno.statSync(libPath);
  console.error("Path already exists: ", libPath);
  Deno.exit(1);
} catch { /**/ }
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
name = "${libName}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
deno_bindgen = "0.8.1"
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
Deno.mkdirSync(libPath + "/.github/workflows", { recursive: true });
const githubAction = await fetch(
  import.meta.resolve("./action.yml"),
).then((r) => r.text()).then((a) => a.replaceAll("$NAME_HERE", libName));
Deno.writeTextFileSync(
  libPath + "/.github/workflows/release.yml",
  githubAction,
);

console.log("1- cd ", libPath);
console.log("2- Run deno_bindgen to develop locally");
console.log("3- Run deno run --unstable mod.ts");
