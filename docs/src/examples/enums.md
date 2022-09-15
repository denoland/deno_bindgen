# Working with `enums`

The macro is capable of generating TypeScript definition and glue code for Rust enums. JS objects are serialized to JSON and converted to an enum using `serde_json`.

### [`src/lib.rs`](#)

```rust
use deno_bindgen::deno_bindgen;

#[deno_bindgen]
pub enum Event {
  Quit,
  MouseMove {
    x: i32,
    y: i32,
  }
}

#[deno_bindgen]
fn do(event: Event) {
  // ...
}
```

### [`mod.ts`](#)

```ts
import { do } from "./bindings/bindings.ts";

do("quit");
do({ mouseMove: { x: 10, y: 10 } })
```

### [`Cargo.toml`](#)

```toml
[package]
name = "enums"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
deno_bindgen = "0.7.0"