## `deno_bindgen`

This tool aims to simplify glue code generation for Deno FFI libraries written
in Rust.

### QuickStart

Annotate on top of Rust `fn`, `struct` and `enum` to make them avaiable to Deno.

```rust
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
}
```

Invoke the CLI to compile and generate bindings:

```shell
$ deno_bindgen
```

And finally import the generated bindings in your JS

```typescript
// add.ts
import { add } from "./bindings/bindings.ts";

add({ a: 1, b: 2 }); // 3
```

### Installation

- Install the `deno_bindgen` CLI with Deno.

```shell
deno install -Afq -n deno_bindgen https://deno.land/x/deno_bindgen/cli.ts
```

Add the following dependencies to your crate.

```toml
# Cargo.toml
[dependencies]
deno_bindgen = "0.5.1"
serde = { version = "1", features = ["derive"] }
```

Change your `crate-type` to `cdylib` and set your package name as well.

```toml
[lib]
name = "___"
crate-type = ["cdylib"]
```

### Bindings

Put `#[deno_bindgen]` on top of a "serde-deriavable" struct, enum or fn.

#### `struct` (named fields)

These transform into Typescript `type`s.

```rust
// lib.rs
#[deno_bindgen]
pub struct A {
  b: Vec<Vec<String>>,
}
```

becomes:

```typescript
// bindings/bindings.ts
export type A = {
  b: Array<Array<string>>;
};
```

#### `enum`

Enums become `type` unions in Typescript.

```rust
#[deno_bindgen]
pub enum Event {
  Quit,
  MouseMove {
    x: i32,
    y: i32,
  }
}
```

becomes:

```typescript
export type Enum =
  | "quit"
  | {
    mouse_move: {
      x: number;
      y: number;
    };
  };
```

### `fn`

Functions are exposed through the FFI boundaries.

```rust
#[deno_bindgen]
fn greet(name: &str) {
  println!("Hello, {}!", name);
}
```

becomes:

```typescript
export function greet(name: string) {
  // ... glue code for calling the
  // symbol.
}
```

Notes

- Use `#[deno_bindgen(non_blocking)]` attribute to call symbol without blocking
  JS event loop. Exposed as an async funtion from bindings.

- Rust doc comments transform to JS docs.
  ```rust
  #[deno_bindgen]
  pub struct Me {
    /// My name...
    /// ...it is
    name: String,
  }
  ```
  becomes:
  ```typescript
  export type Me = {
    /**
     * My name...
     * ...it is
     */
    name: string;
  };
  ```

### CLI

The `deno_bindgen` CLI tool provides the following flags:

- Pass `--release` to create a release build.

- `--release=URL` will load library artifacts from a remote location. This is
  useful for updating bindings for end users after a release:

  ```shell
  deno_bindgen --release=https://github.com/littledivy/deno_sdl2/releases/download/0.2-alpha.1
  ```

  Under the hood this uses [`x/plug`](https://deno.land/x/plug) to fetch and
  cache the artifact.

- Flags after `--` will be passed to `cargo build`. Example:
  ```shell
  deno_bindgen -- --features "cool_stuff"
  ```
