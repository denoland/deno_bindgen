# Multithreaded

Deno FFI has a `nonblocking` mode. You can offload blocking I/O or heavy computation to a seperate thread. The call returns a JavaScript promise that
is resolved when the thread completes.

### [`src/lib.rs`](#)

```rust
use deno_bindgen::deno_bindgen;
use std::{
  thread,
  time::Duration,
};

#[deno_bindgen(nonblocking)]
pub fn sleep(ms: u32) {
  thread::sleep(Duration::from_millis(ms as u64));
}
```

### [`mod.ts`](#)

```ts
import { sleep } from "bindings/bindings.ts";

await sleep(100);
```

### [`Cargo.toml`](#)

```toml
[package]
name = "nonblocking_sleep"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
deno_bindgen = "0.7.0"