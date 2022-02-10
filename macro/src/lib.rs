// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

use deno_bindgen_codegen::function::Function;
use deno_bindgen_codegen::types::TypeDefinition;
use deno_bindgen_codegen::types::tuple::Tuple;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::TypeTuple;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::AttributeArgs;
use syn::FnArg;
use syn::Item;
use syn::ItemFn;
use syn::ItemStruct;

mod attrs;
mod docs;
mod meta;

use crate::meta::Meta;

#[cfg(target_endian = "little")]
const ENDIANNESS: bool = true;

#[cfg(target_endian = "big")]
const ENDIANNESS: bool = false;

thread_local! {
  static META: RefCell<Meta> = RefCell::new(Meta::default());
}

#[proc_macro_attribute]
pub fn deno_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
  let attr = syn::parse_macro_input!(attr as AttributeArgs);
  let item = syn::parse_macro_input!(input as Item);

  META.with(|meta| {
    let mut meta = meta.borrow_mut();

    match item {
      Item::Fn(fn_item) => generate_function(&mut meta, fn_item),
      Item::Struct(struct_item) => generate_struct(&mut meta, struct_item),
      // Item::Enum(_) => todo!(),
      // Item::Type(_) => todo!(),
      // Item::Union(_) => todo!(),
      _ => unimplemented!(),
    }
  })
}

fn syn_type_to_definition(meta: &mut Meta, ) {

}

fn generate_function(meta: &mut Meta, fn_item: ItemFn) -> TokenStream {
  let params = &fn_item.sig.inputs;
  let mut parameters = Vec::with_capacity(params.len());

  for param in params.iter() {
    match param {
      FnArg::Typed(ref val) => {
        let val = val.clone();
        let ty = match *val.ty {
          syn::Type::Path(ref ty) => {
            let ident = ty.path.get_ident().expect("Expected ident").to_string();
            meta.library.lookup_type(&ident).expect(&format!("Could not find {} type", ident))
          }
          // syn::Type::Tuple(ref tuple) => {
          //   tuple.elems.iter().map(|ty| meta.library.lookup_type(ty.path.get_ident()))
          // },
          //syn::Type::Reference(ref ty) => match *ty.elem {
          //  syn::Type::Path(ref ty) => {
          //    let segment = ty.path.segments.first().unwrap();
          //    let ident = segment.ident.to_string();
          //    match ident.as_str() {
          //      "str" => Type::Str,
          //      _ => unimplemented!(),
          //    }
          //  }
          //  syn::Type::Slice(ref slice) => match *slice.elem {
          //    syn::Type::Path(ref path) => {
          //      let segment = path.path.segments.first().unwrap();
          //      let ident = segment.ident.to_string();
          //      match ident.as_str() {
          //        "u8" => {
          //          if ty.mutability.is_some() {
          //            Type::BufferMut
          //          } else {
          //            Type::Buffer
          //          }
          //        }
          //        _ => unimplemented!(),
          //      }
          //    }
          //    _ => unimplemented!(),
          //  },
          //  _ => unimplemented!(),
          //},
          _ => unimplemented!(),
        };
        parameters.push(Some(ty));
      }
      _ => unimplemented!(),
    }
  }
  todo!()
}

fn generate_struct(meta: &mut Meta, struct_item: ItemStruct) -> TokenStream {
  todo!()
}
