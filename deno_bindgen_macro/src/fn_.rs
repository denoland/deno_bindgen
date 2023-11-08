use std::path::Path;

use deno_bindgen_ir::{Symbol, SymbolBuilder, Type};
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
  parse_quote, punctuated::Punctuated, spanned::Spanned, token::Comma, FnArg,
  ItemFn, PatType, ReturnType, TypePath, TypePtr, TypeReference, TypeSlice,
};

use crate::{
  util::{Error, Result},
  FnAttributes,
};

fn custom_type(ty: &str) -> Type {
  // yeah, don't worry about it.
  Type::CustomType(Box::leak(ty.to_string().into_boxed_str()))
}

fn parse_type(ty: &Box<syn::Type>) -> Result<Type> {
  match **ty {
    syn::Type::Path(TypePath { ref path, .. }) => {
      if let Some(ident) = path.get_ident() {
        match ident.to_string().as_str() {
          "u8" => return Ok(Type::Uint8),
          "u16" => return Ok(Type::Uint16),
          "u32" => return Ok(Type::Uint32),
          "u64" => return Ok(Type::Uint64),
          "i8" => return Ok(Type::Int8),
          "i16" => return Ok(Type::Int16),
          "i32" => return Ok(Type::Int32),
          "i64" => return Ok(Type::Int64),
          "f32" => return Ok(Type::Float32),
          "f64" => return Ok(Type::Float64),
          "usize" => return Ok(Type::Uint64),
          "isize" => return Ok(Type::Int64),
          ty_str => {
            return Ok(custom_type(ty_str));
          }
        }
      }

      Err(Error::UnsupportedType)
    }
    syn::Type::Reference(TypeReference { ref elem, .. }) => {
      if let syn::Type::Slice(TypeSlice { ref elem, .. }) = *elem.as_ref() {
        if parse_type(elem)?.is_number() {
          return Ok(Type::Buffer);
        }
      }

      if let syn::Type::Path(TypePath { ref path, .. }) = *elem.as_ref() {
        if let Some(ident) = path.get_ident() {
          let ref ty_str = ident.to_string();
          return Ok(custom_type(ty_str));
        }
      }

      Err(Error::UnsupportedType)
    }

    syn::Type::Ptr(TypePtr { .. }) => Ok(Type::Pointer),
    _ => Err(Error::UnsupportedType),
  }
}

pub fn handle_inner(
  fn_: ItemFn,
  attrs: FnAttributes,
) -> Result<(TokenStream2, SymbolBuilder)> {
  if fn_.sig.asyncness.is_some() {
    return Err(Error::Asyncness);
  }

  if fn_.sig.receiver().is_some() {
    return Err(Error::Reciever);
  }

  // TODO: check right ABI

  let mut ffi_fn = fn_.clone();
  ffi_fn.sig.abi.get_or_insert_with(|| {
    parse_quote!(
        extern "C"
    )
  });

  let mut inputs: Punctuated<FnArg, Comma> = Punctuated::new();
  let mut transforms: Vec<TokenStream2> = Vec::new();

  let mut symbol = SymbolBuilder::new(fn_.sig.ident.clone());
  symbol.non_blocking(attrs.non_blocking);
  symbol.internal(attrs.internal);
  symbol.is_constructor(attrs.constructor);

  // Cannot use enumerate here, there can be multiple raw args per type.
  let mut i = 0;
  for arg in ffi_fn.sig.inputs.iter_mut() {
    match *arg {
      FnArg::Receiver(_) => unreachable!(),
      FnArg::Typed(PatType {
        ref mut pat,
        ref mut ty,
        ..
      }) => {
        let ty = parse_type(ty)?;
        symbol.push(ty);

        const X_ARG_PREFIX: &str = "__arg_";
        // Divide the type into its raw components.
        let raw_ty = ty.raw();

        // Complex types, that need transforms.
        let mut idents = Vec::new();
        for ty in raw_ty {
          let arg_name = Ident::new(
            &format!("{}{}", X_ARG_PREFIX, i),
            Span::mixed_site().located_at(pat.span()),
          );
          inputs.push(parse_quote!(#arg_name: #ty));
          idents.push(arg_name);
          i += 1;
        }

        // Apply the transform.
        if let Some(transform) = ty.apply_arg_transform(pat, &idents) {
          transforms.push(transform);
        }

        // Simple type.
        if raw_ty.len() == 0 {
          inputs.push(arg.clone());
          i += 1;
        }
      }
    }
  }

  let ret_ident = Ident::new("ret", Span::mixed_site());
  let mut ret = Box::new(syn::Pat::Ident(syn::PatIdent {
    attrs: Vec::new(),
    by_ref: None,
    mutability: None,
    ident: ret_ident.clone(),
    subpat: None,
  }));
  let mut ret_transform = TokenStream2::new();
  match ffi_fn.sig.output {
    ReturnType::Default => {}
    ReturnType::Type(_, ref mut ty) => {
      let t = parse_type(ty)?;

      if let Some(transform) =
        t.apply_ret_transform(&mut ret, ret_ident.clone())
      {
        ret_transform = transform;
      }

      symbol.return_type(t);
      **ty = parse_quote!(#t)
    }
  }

  let idents = ffi_fn
    .sig
    .inputs
    .iter()
    .map(|arg| match arg {
      FnArg::Receiver(_) => unreachable!(),
      FnArg::Typed(PatType { ref pat, .. }) => match &**pat {
        syn::Pat::Ident(ident) => ident.ident.clone(),
        _ => unreachable!(),
      },
    })
    .collect::<Vec<_>>();

  let name = fn_.sig.ident.clone();
  ffi_fn.sig.inputs = inputs;

  ffi_fn.block = parse_quote!({
      #fn_

      #(#transforms)*

      let #ret_ident = #name(#(#idents),*);
      #ret_transform
      #ret_ident
  });

  Ok((
    quote::quote! {
        const _: () = {
          #[deno_bindgen::linkme::distributed_slice(deno_bindgen::INVENTORY)]
          pub static _A: deno_bindgen::Inventory = deno_bindgen::Inventory::Symbol(#symbol);
        };

        #[no_mangle]
        #ffi_fn
    },
    symbol,
  ))
}

pub fn handle(fn_: ItemFn, attrs: FnAttributes) -> Result<TokenStream2> {
  let (ffi_fn, _) = handle_inner(fn_, attrs)?;
  Ok(ffi_fn)
}
