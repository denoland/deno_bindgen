## `deno_bindgen`

<img src="./assets/illustration.png" width=200>

This tool aims to simplify glue code generation for Deno FFI libraries written
in Rust.

### QuickStart

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

### Documentation

See the deno_bindgen guide book [bindgen.deno.dev](https://bindgen.deno.dev) for full documentation and examples.


