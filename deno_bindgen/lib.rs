// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

//! ## deno_bindgen
//! This tool aims to simply types & glue code generation for FFI
//! libraries written in Rust.
//!
//! ### Usage
//! Add `serde` and `deno_bindgen` dependency to your crate.
//!
//! ```
//! use deno_bindgen::deno_bindgen;
//!
//! #[deno_bindgen]
//! pub struct Input {
//!   /// Doc comments are transformed into
//!   /// jsdocs.
//!   a: Vec<Vec<String>>,
//! }
//!
//! #[deno_bindgen(non_blocking)]
//! pub fn say_hello(message: &str) {
//!   println!("{}", message);
//! }
//! ```
//!
//! Generated bindings will look like this:
//! ```
//! // bindings/binding.ts
//!
//! // ... <init code here>
//!
//! type Input = {
//!   /**
//!    * Doc comments are transformed into
//!    * jsdocs.
//!    **/
//!   a: Array<Array<string>>;
//! };
//!
//! export async function say_hello(message: string) {
//!   // ... <glue code for symbol here>
//! }
//! ```
//! These bindings contain nessecary code to open the shared library,
//! define symbols and expose type definitions.
//! They can be simply imported into Deno code:
//! ```
//! import { say_hello } from "./bindings/bindings.ts";
//! await say_hello("Demn!")
//! ```
//!
pub use ::serde_json;
pub use deno_bindgen_macro::deno_bindgen;
