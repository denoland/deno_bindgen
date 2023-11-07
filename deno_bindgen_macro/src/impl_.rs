use proc_macro2::TokenStream as TokenStream2;
use syn::ItemImpl;

use crate::util::{self, Result};

pub fn handle(impl_: ItemImpl) -> Result<TokenStream2> {
  if impl_.generics.params.first().is_some() {
    return Err(util::Error::Generics);
  }

  if impl_.generics.where_clause.is_some() {
    return Err(util::Error::WhereClause);
  }

  let self_ty = match *impl_.self_ty {
    syn::Type::Path(ref type_path) => type_path.path.clone(),
    _ => return Err(util::Error::UnsupportedType),
  };

  let ref ty_str @ _ = self_ty.get_ident().unwrap();

  // TODO:
  // - create a new quoted function for each method and codegen using fn_::handle
  // where first arg is self ptr and rest are method args
  // - constructor is a simply special case with no self ptr.
  // - we also need to be aware of &mut self and Self types.

  Ok(quote::quote! {
    #impl_

    const _: () = {
      // Assert that the type implements `BindgenType`.
      const fn _assert_impl<T: ::deno_bindgen::BindgenType>() {}
      _assert_impl::<#ty_str>();

      #[deno_bindgen::linkme::distributed_slice(deno_bindgen::INVENTORY)]
      pub static _B: deno_bindgen::Inventory = deno_bindgen::Inventory::Struct(
        deno_bindgen::inventory::Struct {
          name: stringify!(#ty_str),
          constructor: None,
          methods: &[],
        }
      );
    };
  })
}
