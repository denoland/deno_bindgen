// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

use deno_bindgen_ir::Symbol;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
  meta::ParseNestedMeta, parse2, parse_macro_input, parse_quote, Item, ItemFn,
};

mod fn_;
mod impl_;
mod struct_;
mod util;

#[derive(Default)]
pub(crate) struct FnAttributes {
  pub(crate) non_blocking: bool,
  pub(crate) internal: bool,
}

impl FnAttributes {
  fn parse(&mut self, meta: ParseNestedMeta) -> syn::parse::Result<()> {
    if meta.path.is_ident("non_blocking") {
      self.non_blocking = true;
      Ok(())
    } else {
      Err(meta.error("unsupported attribute"))
    }
  }
}

#[proc_macro_attribute]
pub fn deno_bindgen(args: TokenStream, input: TokenStream) -> TokenStream {
  match parse2::<Item>(input.into()).unwrap() {
    Item::Fn(input) => {
      let mut attrs = FnAttributes::default();
      let attrs_parser = syn::meta::parser(|meta| attrs.parse(meta));
      parse_macro_input!(args with attrs_parser);

      fn_::handle(input, attrs).unwrap().into()
    }
    Item::Struct(input) => struct_::handle(input).unwrap().into(),
    Item::Impl(input) => impl_::handle(input).unwrap().into(),
    _ => panic!("only functions are supported"),
  }
}
