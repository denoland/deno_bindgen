// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

use meta::Symbol;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::quote;
use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::ItemFn;

mod attrs;
mod derive_fn;
mod derive_struct;
mod docs;
mod meta;

use crate::derive_fn::process_function;
use crate::derive_struct::process_struct;
use crate::meta::Glue;
use crate::meta::Type;

const METAFILE: &str = "bindings.json";

#[cfg(target_endian = "little")]
const ENDIANNESS: bool = true;

#[cfg(target_endian = "big")]
const ENDIANNESS: bool = false;

fn transform_params(
  parameters: Vec<Type>,
  params: &mut Vec<TokenStream2>,
  overrides: &mut Vec<TokenStream2>,
  input_idents: &mut Vec<syn::Ident>,
  c_index: &mut usize,
  ident_prefix: Option<&str>,
) {
  let mkident = |c_index: &mut usize| {
    format_ident!(
      "{}_arg{}",
      ident_prefix.unwrap_or("default"),
      c_index.to_string()
    )
  };

  for parameter in parameters {
    match parameter {
      Type::StructEnum { .. } => {
        let ident = mkident(c_index);
        params.push(quote! { #ident: *const u8 });

        *c_index += 1;
        let len_ident = mkident(c_index);
        params.push(quote! { #len_ident: usize });

        overrides.push(quote! {
          let buf = unsafe {
            ::std::slice::from_raw_parts(#ident, #len_ident)
          };
          let #ident = deno_bindgen::serde_json::from_slice(buf).unwrap();
        });

        input_idents.push(ident);
      }
      Type::Str | Type::Buffer | Type::BufferMut => {
        let ident = mkident(c_index);
        match parameter {
          Type::Str | Type::Buffer => params.push(quote! { #ident: *const u8 }),
          Type::BufferMut => params.push(quote! { #ident: *mut u8 }),
          _ => unreachable!(),
        };

        *c_index += 1;
        let len_ident = mkident(c_index);
        params.push(quote! { #len_ident: usize });

        let return_type = match parameter {
          Type::Str => quote! { ::std::str::from_utf8(buf).unwrap() },
          Type::Buffer | Type::BufferMut => quote! { buf },
          _ => unreachable!(),
        };

        let buf_expr = match parameter {
          Type::Str | Type::Buffer => {
            quote! { let buf = ::std::slice::from_raw_parts(#ident, #len_ident); }
          }
          Type::BufferMut => {
            // https://github.com/littledivy/deno_bindgen/issues/26
            // *mut u8 should never outlive the symbol call. This can lead to UB.
            quote! { let mut buf: &'sym mut [u8] = ::std::slice::from_raw_parts_mut(#ident, #len_ident);
            }
          }
          _ => unreachable!(),
        };

        overrides.push(quote! {
          let #ident = unsafe {
            #buf_expr
            #return_type
          };
        });

        input_idents.push(ident);
      }
      Type::Function { symbol, inner } => {
        let syn::TypeBareFn { inputs, output, .. } =
          inner.expect("unreachable");
        let ident = mkident(c_index);

        let mut p_params = vec![];
        let mut p_overrides = vec![];
        let mut p_input_idents = vec![];
        let mut p_c_index = 0;

        let ident_prefix = format!("arg{}", c_index.to_string());
        let (result, transformer) = make_fn(
          *symbol,
          &mut p_params,
          &mut p_overrides,
          &mut p_input_idents,
          &mut p_c_index,
          Some(&ident_prefix),
        );

        let p_overrides = p_overrides
          .iter()
          .fold(quote! {}, |acc, new| quote! { #acc #new });

        let inputs = inputs
          .iter()
          .enumerate()
          .map(|(idx, arg)| {
            let ident = format_ident!("{}_arg{}", ident_prefix, idx);
            let ty = &arg.ty;
            quote! { #ident: #ty, }
          })
          .fold(quote! {}, |acc, new| quote! { #acc #new });

        let inner_ident = format_ident!("{}_inner", ident);
        let static_ident = format_ident!("{}_static", ident);
        let ty = quote! { extern "C" fn (#(#p_params,) *) -> #result };
        overrides.push(quote! {
          static mut #static_ident: Option<#ty> = None;
          fn #inner_ident <'sym> (#inputs) #output {
            #p_overrides
            let result = unsafe { #static_ident .unwrap() (#(#p_input_idents, ) *) };
            #transformer
          };
          unsafe { #static_ident = Some(#ident) };
        });
        params.push(quote! { #ident: #ty });
        input_idents.push(inner_ident);
      }
      // TODO
      _ => {
        let ident = mkident(c_index);
        let ty = syn::Type::from(parameter);
        params.push(quote! { #ident: #ty });
        input_idents.push(ident);
      }
    };

    *c_index += 1;
  }
}

fn make_fn(
  symbol: Symbol,
  params: &mut Vec<TokenStream2>,
  overrides: &mut Vec<TokenStream2>,
  input_idents: &mut Vec<syn::Ident>,
  c_index: &mut usize,
  ident_prefix: Option<&str>,
) -> (syn::Type, TokenStream2) {
  transform_params(
    symbol.parameters,
    params,
    overrides,
    input_idents,
    c_index,
    ident_prefix,
  );

  match symbol.result {
    Type::Buffer
    // Note that this refers to an owned String
    // and not a `&str`
    | Type::Str => {
      let ty = parse_quote! { *const u8 };
      let slice = match symbol.result {
        Type::Str => quote! {
          result.as_bytes()
        },
        _ => quote! { result }
      };
      let transformer = quote! {
        let length = (result.len() as u32).to_be_bytes();
        let mut v = length.to_vec();
        v.extend_from_slice(#slice);

        ::std::mem::forget(result);
        let result = v.as_ptr();
        // Leak the result to JS land.
        ::std::mem::forget(v);
        result
      };

      (ty, transformer)
    }
    Type::StructEnum { .. } => {
      let ty = parse_quote! { *const u8 };
      let transformer = quote! {
        let json = deno_bindgen::serde_json::to_string(&result).expect("Failed to serialize as JSON");
        let encoded = json.into_bytes();
        let length = (encoded.len() as u32).to_be_bytes();
        let mut v = length.to_vec();
        v.extend(encoded.clone());

        let ret = v.as_ptr();
        // Leak the result to JS land.
        ::std::mem::forget(v);
        ret
      };

      (ty, transformer)
    }
    Type::Ptr => (parse_quote! { *const u8 }, quote! { result }),
    _ => (syn::Type::from(symbol.result), quote! { result }),
  }
}

#[proc_macro_attribute]
pub fn deno_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
  let mut metadata: Glue = match OpenOptions::new().read(true).open(METAFILE) {
    Ok(mut fd) => {
      let mut meta = String::new();
      fd.read_to_string(&mut meta)
        .expect("Error reading meta file");

      serde_json::from_str(&meta).unwrap_or_default()
    }
    Err(_) => Glue {
      little_endian: ENDIANNESS,
      name: env::var("CARGO_CRATE_NAME").unwrap_or_default(),
      ..Default::default()
    },
  };

  let mut metafile = OpenOptions::new()
    .write(true)
    .create(true)
    .open(METAFILE)
    .expect("Error opening meta file");

  match syn::parse::<ItemFn>(input.clone()) {
    Ok(func) => {
      let attr = parse_macro_input!(attr as syn::AttributeArgs);
      let symbol = process_function(func.clone(), attr, &mut metadata).unwrap();
      let mut params = vec![];
      let mut overrides = vec![];
      let mut input_idents = vec![];
      let mut c_index = 0;

      let (result, transformer) = make_fn(
        symbol,
        &mut params,
        &mut overrides,
        &mut input_idents,
        &mut c_index,
        None,
      );
      metafile
        .write_all(&serde_json::to_vec(&metadata).unwrap())
        .unwrap();

      let name = &func.sig.ident;
      let fn_inputs = &func.sig.inputs;
      let fn_output = &func.sig.output;
      let fn_generics = &func.sig.generics;
      let fn_block = &func.block;

      let overrides = overrides
        .iter()
        .fold(quote! {}, |acc, new| quote! { #acc #new });

      TokenStream::from(quote! {
        #[no_mangle]
        pub extern "C" fn #name <'sym> (#(#params,) *) -> #result {
          fn __inner_impl #fn_generics (#fn_inputs) #fn_output #fn_block
          #overrides
          let result = __inner_impl(#(#input_idents, ) *);
          #transformer
        }
      })
    }
    Err(_) => {
      let input = syn::parse_macro_input!(input as syn::DeriveInput);
      process_struct(&mut metadata, input.clone()).unwrap();

      metafile
        .write_all(&serde_json::to_vec(&metadata).unwrap())
        .unwrap();

      TokenStream::from(quote! {
        #[derive(::serde::Deserialize,::serde::Serialize)]
        #input
      })
    }
  }
}
