// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

use proc_macro::TokenStream;
use syn::meta::ParseNestedMeta;
use syn::parse2;
use syn::parse_macro_input;
use syn::Item;

mod fn_;
mod impl_;
mod struct_;
mod util;

#[derive(Default)]
pub(crate) struct FnAttributes {
  pub(crate) non_blocking: bool,
  pub(crate) constructor: bool,

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

#[cfg(test)]
mod tests {
  use std::path::PathBuf;

  #[testing_macros::fixture("tests/fn/*.test.rs")]
  fn test_codegen_fn(input: PathBuf) {
    let update_expected = std::env::var("UPDATE_EXPECTED").is_ok();

    let source =
      std::fs::read_to_string(&input).expect("failed to read test case");
    let item_fn = syn::parse_str::<syn::ItemFn>(&source)
      .expect("failed to parse test case");

    let tokens = crate::fn_::handle(item_fn, Default::default()).unwrap();
    let tree = syn::parse2(tokens).unwrap();
    let actual = prettyplease::unparse(&tree);

    let expected_out = input.with_extension("out.rs");
    if update_expected {
      std::fs::write(expected_out, actual)
        .expect("Failed to write expectation file");
    } else {
      let expected = std::fs::read_to_string(expected_out)
        .expect("Failed to read expectation file");
      assert_eq!(
        expected, actual,
        "Failed to match expectation. Use UPDATE_EXPECTED=1."
      );
    }
  }
}
