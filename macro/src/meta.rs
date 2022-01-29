// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

use std::env;

use deno_bindgen_codegen::{library::Library, loader::deno::DenoLoader};
use syn::{ext::IdentExt, parse_quote};

// impl From<Type> for syn::Type {
//   fn from(ty: Type) -> Self {
//     match ty {
//       Type::I8 => parse_quote! { i8 },
//       Type::U8 => parse_quote! { u8 },
//       Type::I16 => parse_quote! { i16 },
//       Type::U16 => parse_quote! { u16 },
//       Type::I32 => parse_quote! { i32 },
//       Type::U32 => parse_quote! { u32 },
//       Type::I64 => parse_quote! { i64 },
//       Type::U64 => parse_quote! { u64 },
//       Type::F32 => parse_quote! { f32 },
//       Type::F64 => parse_quote! { f64 },
//       Type::Usize => parse_quote! { usize },
//       Type::Isize => parse_quote! { isize },
//       Type::Void => parse_quote! { () },
//       _ => unreachable!(),
//     }
//   }
// }

pub struct Meta {
  pub library: Library,
}

impl Meta {
  pub fn new(library: Library) -> Self {
    Self { library }
  }
}

impl Default for Meta {
  fn default() -> Self {
    Self::new(Library::new(
      None,
      Box::new(DenoLoader::new(
        false,
        &env::var("CARGO_TARGET_DIR").expect("Expected CARGO_TARGET_DIR"),
      )),
    ))
  }
}
