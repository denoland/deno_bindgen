// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use syn::parse_quote;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
  /// Straight forward types supported
  /// by Deno's FFI
  I8,
  U8,
  I16,
  U16,
  I32,
  U32,
  I64,
  U64,
  F32,
  F64,
  Usize,
  Isize,
  Void,

  /// Types that pave way for
  /// serializers. buffers <3
  Buffer,
  BufferMut,
  Str,
  Ptr,

  /// Not-so straightforward types that
  /// `deno_bingen` maps to.
  StructEnum {
    ident: String,
  },
}

#[repr(u8)]
pub enum CType {
  /// Straight forward types supported
  /// by Deno's FFI
  I8,
  U8,
  I16,
  U16,
  I32,
  U32,
  I64,
  U64,
  F32,
  F64,
  Usize,
  Isize,
  Void,
  Buffer,
  BufferMut,
  Str,
  Ptr,
}

impl From<&Type> for CType {
  fn from(ty: &Type) -> Self {
    match ty {
      Type::I8 => CType::I8,
      Type::U8 => CType::U8,
      Type::I16 => CType::I16,
      Type::U16 => CType::U16,
      Type::I32 => CType::I32,
      Type::U32 => CType::U32,
      Type::I64 => CType::I64,
      Type::U64 => CType::U64,
      Type::F32 => CType::F32,
      Type::F64 => CType::F64,
      Type::Usize => CType::Usize,
      Type::Isize => CType::Isize,
      Type::Void => CType::Void,
      Type::Buffer => CType::Buffer,
      Type::BufferMut => CType::BufferMut,
      Type::Str => CType::Str,
      Type::Ptr => CType::Ptr,
      Type::StructEnum { .. } => CType::Ptr,
    }
  }
}

impl From<Type> for syn::Type {
  fn from(ty: Type) -> Self {
    match ty {
      Type::I8 => parse_quote! { i8 },
      Type::U8 => parse_quote! { u8 },
      Type::I16 => parse_quote! { i16 },
      Type::U16 => parse_quote! { u16 },
      Type::I32 => parse_quote! { i32 },
      Type::U32 => parse_quote! { u32 },
      Type::I64 => parse_quote! { i64 },
      Type::U64 => parse_quote! { u64 },
      Type::F32 => parse_quote! { f32 },
      Type::F64 => parse_quote! { f64 },
      Type::Usize => parse_quote! { usize },
      Type::Isize => parse_quote! { isize },
      Type::Void => parse_quote! { () },
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
  pub parameters: Vec<Type>,
  pub result: Type,
  pub non_blocking: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Glue {
  pub name: String,
  pub little_endian: bool,
  pub symbols: HashMap<String, Symbol>,
  pub type_defs: HashMap<String, HashMap<String, String>>,
  pub ts_types: HashMap<String, String>,
}
