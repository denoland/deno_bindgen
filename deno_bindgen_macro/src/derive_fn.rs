// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

use crate::meta::Glue;
use crate::meta::Symbol;
use crate::meta::Type;
use syn::AttributeArgs;
use syn::FnArg;
use syn::ItemFn;
use syn::Meta;
use syn::NestedMeta;
use syn::ReturnType;

fn make_args(ty: &syn::Type, metadata: &mut Glue) -> Type {
  match ty {
    syn::Type::Path(ref ty) => {
      let segment = ty.path.segments.first().unwrap();
      let ident = segment.ident.to_string();

      match ident.as_str() {
        "i8" => Type::I8,
        "u8" => Type::U8,
        "i16" => Type::I16,
        "u16" => Type::U16,
        "i32" => Type::I32,
        "u32" => Type::U32,
        "i64" => Type::I64,
        "u64" => Type::U64,
        "usize" => Type::Usize,
        "isize" => Type::Isize,
        _ => {
          metadata.type_defs.get(&ident).expect(&format!(
            "Type definition not found for `{}` identifier",
            &ident,
          ));

          Type::StructEnum { ident }
        }
      }
    }
    // eg: fn(usize) -> bool
    syn::Type::BareFn(ref ty) => {
      let mut parameters = Vec::with_capacity(ty.inputs.len());
      for param in ty.inputs.iter() {
        let ty = make_args(&param.ty, metadata);
        parameters.push(ty);
      }
      let result = make_ret(&ty.output, metadata);
      let symbol = Symbol {
        parameters,
        result,
        non_blocking: false,
      };
      Type::Function {
        symbol: Box::new(symbol),
        inner: Some(ty.clone()),
      }
    }
    syn::Type::Reference(ref ty) => match *ty.elem {
      syn::Type::Path(ref ty) => {
        let segment = ty.path.segments.first().unwrap();
        let ident = segment.ident.to_string();

        match ident.as_str() {
          "str" => Type::Str,
          _ => unimplemented!(),
        }
      }
      syn::Type::Slice(ref slice) => match *slice.elem {
        syn::Type::Path(ref path) => {
          let segment = path.path.segments.first().unwrap();
          let ident = segment.ident.to_string();

          match ident.as_str() {
            "u8" => {
              if ty.mutability.is_some() {
                Type::BufferMut
              } else {
                Type::Buffer
              }
            }
            _ => unimplemented!(),
          }
        }
        _ => unimplemented!(),
      },
      _ => unimplemented!(),
    },
    _ => unimplemented!(),
  }
}

// TODO(@littledivy): Support function pointers.
fn make_ret(ty: &syn::ReturnType, metadata: &mut Glue) -> Type {
  match ty {
    ReturnType::Default => Type::Void,
    ReturnType::Type(_, ref ty) => match ty.as_ref() {
      syn::Type::Ptr(_) => Type::Ptr,
      syn::Type::Path(ref ty) => {
        let segment = ty.path.segments.first().unwrap();
        let ident = segment.ident.to_string();

        match ident.as_str() {
          "i8" => Type::I8,
          "u8" => Type::U8,
          "i16" => Type::I16,
          "u16" => Type::U16,
          "i32" => Type::I32,
          "u32" => Type::U32,
          "i64" => Type::I64,
          "u64" => Type::U64,
          "usize" => Type::Usize,
          "isize" => Type::Isize,
          "f32" => Type::F32,
          "f64" => Type::F64,
          "String" => Type::Str,
          _ => match metadata.type_defs.get(&ident) {
            Some(_) => Type::StructEnum { ident },
            None => panic!("{} return type not supported by Deno FFI", ident),
          },
        }
      }
      syn::Type::Reference(ref ty) => match *ty.elem {
        syn::Type::Slice(ref slice) => match *slice.elem {
          syn::Type::Path(ref path) => {
            let segment = path.path.segments.first().unwrap();
            let ident = segment.ident.to_string();

            match ident.as_str() {
              "u8" => {
                if ty.mutability.is_some() {
                  // https://github.com/denoland/deno_bindgen/issues/39
                  panic!("Mutable slices are not mutable from JS");
                } else {
                  Type::Buffer
                }
              }
              _ => unimplemented!(),
            }
          }
          _ => unimplemented!(),
        },
        _ => unimplemented!(),
      },
      _ => unimplemented!(),
    },
  }
}

pub fn process_function(
  function: ItemFn,
  attr: AttributeArgs,
  metadata: &mut Glue,
) -> Result<Symbol, String> {
  let params = &function.sig.inputs;
  let mut parameters = Vec::with_capacity(params.len());

  for param in params.iter() {
    match param {
      FnArg::Typed(ref val) => {
        let ty = make_args(&val.ty, metadata);
        parameters.push(ty);
      }
      _ => unimplemented!(),
    }
  }

  let result = make_ret(&function.sig.output, metadata);

  let symbol_name = function.sig.ident.to_string();
  let non_blocking = match attr.get(0).as_ref() {
    Some(NestedMeta::Meta(Meta::Path(ref attr_ident))) => {
      attr_ident.is_ident("non_blocking")
    }
    _ => false,
  };

  let symbol = Symbol {
    parameters,
    result,
    non_blocking,
  };
  metadata.symbols.insert(symbol_name, symbol.clone());

  Ok(symbol)
}
