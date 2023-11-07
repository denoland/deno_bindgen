// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.
pub use ::serde_json;
use deno_bindgen_ir::codegen::Options;
pub use deno_bindgen_ir::*;
pub use deno_bindgen_macro::deno_bindgen;
pub use linkme;
use linkme::distributed_slice;

#[distributed_slice]
pub static INVENTORY: [Inventory];

pub trait BindgenType {
  fn type_name() -> &'static str;
}

#[no_mangle]
fn init_deno_bindgen(opt: Options) {
  deno_bindgen_ir::codegen::generate(&INVENTORY, opt).unwrap();
}
