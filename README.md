# `deno_bindgen`

<img align="right" src="./assets/illustration.png" width=200>

This tool aims to simplify glue code generation for Deno FFI libraries written
in Rust.

## Install

Install the command-line via `cargo`:

```bash
cargo install deno_bindgen_cli
```

## Usage

```rust
use deno_bindgen::deno_bindgen;

// Export `add` function to JavaScript.
#[deno_bindgen]
fn add(a: u32, b: u32) -> u32 {
    a + b
}
```

Use the exported functions directly in ESM with TypeScript typings

```typescript
import { add } from "@ffi/example";

add(1, 2);
```

## Design

The tool is designed to make it very easy to write high performance FFI
bindings. A lot of the things have been redesigned in `0.10` to prevent perf
footguns.

TypeScript types are generated and supported OOTB.

All class handles support disposing memory via the Explicit Resource Management
API (`using`).

```rust
#[deno_bindgen]
pub struct Foo;

#[deno_bindgen]
impl Foo {
  #[constructor]
  pub fn new() -> Self {
    Self
  }

  pub fn bar(&self) {
    // ...
  }
}
```

```js
import { Foo } from "@ffi/example";

{
  using foo = new Foo();
  foo.bar();
  // foo is disposed here...
}
```

High performance. Codegen tries its best to take the fastest possible path for
all bindings as-if they were written by hand to properly leverage the power of
the Deno FFI JIT calls.

```
> make bench
cpu: Apple M1
runtime: deno 1.38.0 (aarch64-apple-darwin)

file:///Users/divy/gh/deno_bindgen/example/bench.js
benchmark      time (avg)        iter/s             (min … max)       p75       p99      p995
--------------------------------------------------------------- -----------------------------
add             6.88 ns/iter 145,297,626.6    (6.78 ns … 13.33 ns)   6.81 ns   8.22 ns    9.4 ns
bytelen         8.05 ns/iter 124,278,976.3     (7.81 ns … 18.1 ns)   8.09 ns  10.39 ns  11.64 ns
```
