use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_quote, ImplItemFn, ItemImpl, parse, punctuated::Punctuated};

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

  let mut methods = Vec::new();
  let mut syms = Punctuated::<TokenStream2, syn::Token![,]>::new();
  for item in impl_.items.iter() {
    match item {
      syn::ImplItem::Fn(ImplItemFn { sig, .. }) => {
        if sig.receiver().is_some() {
          let ref method_name = sig.ident;
          let ref out = sig.output;
          let inputs = sig.inputs.iter().skip(1).collect::<Vec<_>>();
          let idents = inputs
            .iter()
            .map(|arg| match arg {
              syn::FnArg::Receiver(_) => unreachable!(),
              syn::FnArg::Typed(pat_type) => match &*pat_type.pat {
                syn::Pat::Ident(ident) => ident.ident.clone(),
                _ => unreachable!(),
              },
            })
            .collect::<Vec<_>>();
          let method = parse_quote! {
            fn #method_name (self_: *mut #ty_str, #(#inputs),*) #out {
              let self_ = unsafe { &mut *self_ };
              self_. #method_name (#(#idents),*)
            }
          };
          
          let (generated, sym) = crate::fn_::handle_inner(method, crate::FnAttributes { 
            internal: true,
            ..Default::default()
           })?;
          methods.push(generated);
          syms.push(quote::quote! { #sym });
        }
      }
      _ => {}
    }
  }

  // TODO:
  // - create a new quoted function for each method and codegen using fn_::handle
  // where first arg is self ptr and rest are method args
  // - constructor is a simply special case with no self ptr.
  // - we also need to be aware of &mut self and Self types.

  Ok(quote::quote! {
    #impl_
    #(#methods)*
    const _: () = {
      // Assert that the type implements `BindgenType`.
      const fn _assert_impl<T: ::deno_bindgen::BindgenType>() {}
      _assert_impl::<#ty_str>();

      #[deno_bindgen::linkme::distributed_slice(deno_bindgen::INVENTORY)]
      pub static _B: deno_bindgen::Inventory = deno_bindgen::Inventory::Struct(
        deno_bindgen::inventory::Struct {
          name: stringify!(#ty_str),
          constructor: None,
          methods: &[#syms],
        }
      );
    };
  })
}
