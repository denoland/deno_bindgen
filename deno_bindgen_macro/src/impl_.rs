use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::ImplItemFn;
use syn::ItemImpl;

use crate::util::Result;
use crate::util::{self};

pub fn handle(mut impl_: ItemImpl) -> Result<TokenStream2> {
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
  for item in impl_.items.iter_mut() {
    match item {
      syn::ImplItem::Fn(ImplItemFn { sig, attrs, .. }) => {
        let mut is_constructor = false;
        if let Some(attr) = attrs.first() {
          let path = attr.path();
          is_constructor = path.is_ident("constructor");

          attrs.clear();
        }

        // TODO: Add common name magling util.
        let method_name = sig.ident.clone();
        let mangled_name = format_ident!("__{}_{}", ty_str, method_name);
        // ...
        let ref out = sig.output;
        let inputs = sig.inputs.iter();

        fn idents_with_skip<'a>(
          arg: syn::punctuated::Iter<'a, syn::FnArg>,
          skip: usize,
        ) -> Vec<&'a syn::Ident> {
          arg
            .skip(skip)
            .map(|arg| match arg {
              syn::FnArg::Receiver(_) => unreachable!(),
              syn::FnArg::Typed(pat_type) => match &*pat_type.pat {
                syn::Pat::Ident(ident) => &ident.ident,
                _ => unreachable!(),
              },
            })
            .collect::<Vec<_>>()
        }

        let method = if sig.receiver().is_some() {
          let idents = idents_with_skip(inputs.clone(), 1);
          // First argument is the receiver, we skip it.
          let inputs = inputs.skip(1);

          parse_quote! {
            #[allow(non_snake_case)]
            fn #mangled_name (self_: *mut #ty_str, #(#inputs),*) #out {
              let self_ = unsafe { &mut *self_ };
              self_. #method_name (#(#idents),*)
            }
          }
        } else if is_constructor {
          let idents = idents_with_skip(inputs.clone(), 0);
          parse_quote!(
            #[allow(non_snake_case)]
            fn #mangled_name (#(#inputs),*) #out {
              #ty_str:: #method_name (#(#idents),*)
            }
          )
        } else {
          return Err(util::Error::MissingReceiver);
        };

        let (generated, mut sym) = crate::fn_::handle_inner(
          method,
          crate::FnAttributes {
            internal: true,
            constructor: is_constructor,
            ..Default::default()
          },
        )?;

        // Set method name to the original name as the
        // managed name is used for the internal symbol.
        sym.set_name(method_name);

        methods.push(generated);
        syms.push(quote::quote! { #sym });
      }
      _ => {}
    }
  }

  // Generate a dealloc method.
  {
    let ident = format_ident!("__{}_dealloc", ty_str);
    let dispose = parse_quote! {
      #[allow(non_snake_case)]
      fn #ident(self_: *mut #ty_str) {
        if self_.is_null() {
          return;
        }
        unsafe { drop(Box::from_raw(self_)) }
      }
    };
    let (generated, mut sym) = crate::fn_::handle_inner(
      dispose,
      crate::FnAttributes {
        internal: true,
        ..Default::default()
      },
    )?;

    sym.set_name(format_ident!("dealloc"));

    methods.push(generated);
    syms.push(quote::quote! { #sym });
  }

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
          methods: &[#syms],
        }
      );
    };
  })
}
