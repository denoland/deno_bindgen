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
Deno.mkdirSync(libPath + "/.github/workflows", { recursive: true });
Deno.writeTextFileSync(
  libPath + "/.github/workflows/release.yml",
  `\
name: Release libs

on:
  workflow_dispatch:
    inputs:
      tag:
        description: "tag name"
        required: true

jobs:
  build:
    name: Release libs
    runs-on: \${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]

    steps:
    - uses: actions/checkout@v2

    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    #uncomment to build for macos arm
    #- uses: goto-bus-stop/setup-zig@v1
    #  with:
    #    version: 0.9.0

    - name: Build
      uses: actions-rs/cargo@v1
      with:
        command: build
        args: --release

    - name: Build MacOS x86_64
      if: runner.os == 'MacOS'
      run: |
        mv target/release/lib${libName}.dylib lib${libName}_x86_64.dylib

    - name: Upload MacOS x86_64
      if: runner.os == 'MacOS'
      uses: svenstaro/upload-release-action@v2
      with:
        file: lib${libName}_x86_64.dylib
        tag: \${{ github.event.inputs.tag }}
        overwrite: true

    #uncomment to build for macos arm
    #- name: Build MacOS aarch64 lib
    #  if: runner.os == 'Linux'
    #  run: |
    #    rustup target add aarch64-apple-darwin
    #    cargo install cargo-zigbuild
    #    cargo zigbuild --release --target aarch64-apple-darwin
    #    mv target/aarch64-apple-darwin/release/lib${libName}.dylib lib${libName}_aarch64.dylib

    - name: Upload MacOS aarch64
      if: runner.os == 'Linux'
      uses: svenstaro/upload-release-action@v2
      with:
        file: lib${libName}_aarch64.dylib
        tag: \${{ github.event.inputs.tag }}
        overwrite: true

    - name: Release Linux lib
      if: runner.os == 'Linux'
      uses: svenstaro/upload-release-action@v2
      with:
        file: target/release/lib${libName}.so
        tag: \${{ github.event.inputs.tag }}
        overwrite: true

    - name: Release Windows lib
      if: runner.os == 'Windows'
      uses: svenstaro/upload-release-action@v2
      with:
        file: target/release/${libName}.dll
        tag: \${{ github.event.inputs.tag }}
        overwrite: true`,
);

console.log("1- cd ", libPath);
console.log(
  "2- If you're developing locally run deno_bindgen\n" +
    "If you want to create release bindings, you can trigger a github action build manually in {github_repo}/actions\n" +
    "Then use deno_bindgen --release={github_repo}/releases/download/{tag_name}",
);
console.log("3- Run deno run --unstable lib.ts");
