# Hello, World!

### [`src/lib.rs`](#)

```rust
use deno_bindgen::deno_bindgen;

#[deno_bindgen]
pub fn greet(name: &str) {
  println!("Hello, {}!", name);
}
```

### [`mod.ts`](#)

```ts
import { greet } from "bindings/bindings.ts";

greet("Deno");
```

### [`Cargo.toml`](#)

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
deno_bindgen = "0.7.0"
```