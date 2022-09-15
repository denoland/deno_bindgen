# Installation

* Install Rust using `rustup` and Deno.
* Install `deno_bindgen` CLI:

```
deno install -Afq -n deno_bindgen https://deno.land/x/deno_bindgen/cli.ts
```

* Add the following dependencies to your crate.

```toml
[dependencies]
deno_bindgen = "0.7.0"
# (Optional): only when you are using `struct` and `enum`s.
serde = { version = "1", features = ["derive"] }
```

* Change your `crate-type` to `cdylib` and set your package name as well.

```toml
[lib]
name = "my_lib"
crate-type = ["cdylib"]
```