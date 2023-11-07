use proc_macro2::TokenStream as TokenStream2;
use syn::ItemStruct;

use crate::util::{self, Result};

pub fn handle(struct_: ItemStruct) -> Result<TokenStream2> {
  if struct_.generics.params.first().is_some() {
    return Err(util::Error::Generics);
  }

  if struct_.generics.where_clause.is_some() {
    return Err(util::Error::WhereClause);
  }

  let ref ty_str @ _ = struct_.ident;
  Ok(quote::quote! {
    #struct_

    impl ::deno_bindgen::BindgenType for #ty_str {
        fn type_name() -> &'static str {
            stringify!(#ty_str)
        }
    }
  })
}
