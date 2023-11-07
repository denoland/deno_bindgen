## `deno_bindgen`

<img src="./assets/illustration.png" width=200>

This tool aims to simplify glue code generation for Deno FFI libraries written
in Rust.

```rust
use deno_bindgen::deno_bindgen;

#[deno_bindgen]
fn add(a: u32, b: u32) -> u32 {
    a + b
}
```

```typescript
import { add } from "@ffi/example";

add(1, 2);
```