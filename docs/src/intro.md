# Introduction

This book is a guide to using `deno_bindgen`, a Rust library and CLI tool to write high-level Deno FFI libraries. It's recommended that you be familar with
the concept of FFI. For more in-depth knowledge of how Deno's FFI interface works, read the [Deno Manual](https://deno.land/manual/runtime/ffi_api) and [Denonomicon](https://denonomicon.deno.dev/).

**Writing FFI code can be hard.** There are multiple things you **must** be very careful with. Here's an example passing JS strings to Rust:

```rust
#[no_mangle]
pub extern "C" fn say_hello(ptr: *const u8, len: usize) {
    // ...
}
```

```js
const { symbols: { say_hello } } = Deno.dlopen("libtest.so", {
  parameters: ["buffer", "usize"],
  result: "void",
});
const encoded = new TextEncoder().encode("Hello World");
say_hello(encoded, encoded.byteLength);
```

Due to the nature of FFI, many things can go wrong here like wrong parameter types, incorrect byte length, etc.

A common glue code generator like `deno_bindgen` hides away all of these unsafe layers to expose a safe-ish API where the possibility of user error is very limited.

```rust
#[deno_bindgen]
pub fn say_hello(input: &str) { // Zero copy with Rust lifetime safety guarantees.
  // ...
}
```

```js
import { say_hello } from "./bindings/binding.ts";
say_hello("Hello World");
```

**Maintaining FFI code can be hard**

As a library author, you'd want to manage distribution of prebuilt libraries for your users. A common practice in the Deno ecosystem is to 
use a CI/CD service with Github releases. `deno_bindgen --release` can be used to generate code during library release that will automatically
cache the shared library on user's machine.