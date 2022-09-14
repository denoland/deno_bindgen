# Working with `struct`

The macro is capable of generating TypeScript definition and glue code for Rust structs. JS objects are serialized to JSON and converted to a struct.

### [`src/lib.rs`](#)

```rust
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

### [`mod.ts`](#)

```ts
import { add } from "./bindings/bindings.ts";

add({ a: 1, b: 2 }); // 3
```

### [`Cargo.toml`](#)

```toml
[package]
name = "add_object"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
deno_bindgen = "0.7.0"