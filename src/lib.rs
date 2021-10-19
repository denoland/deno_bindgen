#![feature(box_patterns)]

use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::Data;
use syn::DataStruct;
use syn::Fields;
use syn::ItemFn;
use syn::Meta;
use syn::NestedMeta;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
enum Type {
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
  Str,

  /// Not-so straightforward types that
  /// `deno_bingen` maps to.
  StructEnum {
    ident: String,
  },
  // XXX: We need this for
  // transmute "backend".
  // { len: usize }
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
      _ => panic!("Unreachable"),
    }
  }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Symbol {
  parameters: Vec<Type>,
  result: Type,
  non_blocking: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Glue {
  name: String,
  little_endian: bool,
  symbols: HashMap<String, Symbol>,
  type_defs: HashMap<String, HashMap<String, String>>,
  ts_types: HashMap<String, String>,
}

const METAFILE: &str = "bindings.json";

#[cfg(target_endian = "little")]
const ENDIANNESS: bool = true;

#[cfg(target_endian = "big")]
const ENDIANNESS: bool = false;

#[proc_macro_attribute]
pub fn deno_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
  let mut metadata: Glue = match OpenOptions::new().read(true).open(METAFILE) {
    Ok(mut fd) => {
      let mut meta = String::new();
      fd.read_to_string(&mut meta)
        .expect("Error reading meta file");

      serde_json::from_str(&meta).unwrap()
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

  let (func, symbol) = match syn::parse::<ItemFn>(input.clone()) {
    Ok(func) => {
      let attr = parse_macro_input!(attr as syn::AttributeArgs);
      let symbol = process_function(func.clone(), attr, &mut metadata).unwrap();
      (func, symbol)
    }
    Err(_) => {
      let input = syn::parse_macro_input!(input as syn::DeriveInput);
      process_struct(&mut metadata, input.clone()).unwrap();

      metafile
        .write_all(&serde_json::to_vec(&metadata).unwrap())
        .unwrap();
      let name = &input.ident;

      return TokenStream::from(quote! {
        #[derive(serde::Deserialize)]
        #input
      });
    }
  };

  let mut params = vec![];
  let mut overrides = vec![];
  let mut input_idents = vec![];
  let mut c_index = 0;

  for parameter in symbol.parameters {
    match parameter {
      Type::StructEnum { .. } => {
        let ident = format_ident!("arg{}", c_index.to_string());
        params.push(quote! { #ident: *const u8 });

        c_index += 1;
        let len_ident = format_ident!("arg{}", c_index.to_string());
        params.push(quote! { #len_ident: usize });

        overrides.push(quote! {
          let buf = unsafe {
            ::std::slice::from_raw_parts(#ident, #len_ident)
          };
          let #ident = serde_json::from_slice(buf).unwrap();
        });

        input_idents.push(ident);
      }
      Type::Str | Type::Buffer => {
        let ident = format_ident!("arg{}", c_index.to_string());
        params.push(quote! { #ident: *const u8 });

        c_index += 1;
        let len_ident = format_ident!("arg{}", c_index.to_string());
        params.push(quote! { #len_ident: usize });

        let return_type = match parameter {
          Type::Str => quote! { ::std::str::from_utf8(buf).unwrap() },
          Type::Buffer => quote! { buf },
          _ => unreachable!(),
        };

        overrides.push(quote! {
          let #ident = unsafe {
            let buf = ::std::slice::from_raw_parts(#ident, #len_ident);
            #return_type
          };
        });

        input_idents.push(ident);
      }
      // TODO
      _ => {
        let ident = format_ident!("arg{}", c_index.to_string());
        let ty = syn::Type::from(parameter);
        params.push(quote! { #ident: #ty });
        input_idents.push(ident);
      }
    };

    c_index += 1;
  }

  let name = &func.sig.ident;
  let fn_inputs = &func.sig.inputs;
  let fn_output = &func.sig.output;
  let fn_block = &func.block;

  let overrides = overrides
    .iter()
    .fold(quote! {}, |acc, new| quote! { #acc #new });

  metafile
    .write_all(&serde_json::to_vec(&metadata).unwrap())
    .unwrap();

  TokenStream::from(quote! {
    #[no_mangle]
    pub extern "C" fn #name (#(#params,) *) #fn_output {
      fn __inner_impl (#fn_inputs) #fn_output #fn_block
      #overrides
      __inner_impl(#(#input_idents, ) *)
    }
  })
}

fn process_function(
  function: ItemFn,
  attr: syn::AttributeArgs,
  metadata: &mut Glue,
) -> Result<Symbol, String> {
  let params = &function.sig.inputs;
  let mut parameters = Vec::with_capacity(params.len());

  for param in params.iter() {
    match param {
      syn::FnArg::Typed(ref val) => {
        let val = val.clone();
        let ty = match *val.ty {
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
          syn::Type::Reference(ref ty) => match *ty.elem {
            syn::Type::Path(ref ty) => {
              let segment = ty.path.segments.first().unwrap();
              let ident = segment.ident.to_string();

              match ident.as_str() {
                "str" => Type::Str,
                _ => unimplemented!(),
              }
            }
            syn::Type::Slice(ref ty) => match *ty.elem {
              syn::Type::Path(ref ty) => {
                let segment = ty.path.segments.first().unwrap();
                let ident = segment.ident.to_string();

                match ident.as_str() {
                  "u8" => Type::Buffer,
                  _ => unimplemented!(),
                }
              }
              _ => unimplemented!(),
            },
            _ => unimplemented!(),
          },
          _ => unimplemented!(),
        };

        parameters.push(ty);
      }
      _ => unimplemented!(),
    }
  }

  let result = match &function.sig.output {
    syn::ReturnType::Default => Type::Void,
    syn::ReturnType::Type(_, box syn::Type::Path(ty)) => {
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
        _ => panic!("{} return type not supported by Deno FFI", ident),
      }
    }
    _ => unimplemented!(),
  };

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

fn process_struct(
  metadata: &mut Glue,
  input: syn::DeriveInput,
) -> Result<(), String> {
  match &input.data {
    Data::Struct(DataStruct {
      fields: Fields::Named(fields),
      ..
    }) => {
      let fields = &fields.named;

      let name = &input.ident;
      let mut fmap = HashMap::new();
      let mut typescript: Vec<String> = vec![];

      for field in fields.iter() {
        let ident = field
          .ident
          .as_ref()
          .expect("Field without ident")
          .to_string();

        match field.ty {
          syn::Type::Path(ref ty) => {
            let segment = &ty.path.segments.first().unwrap();
            let ty = segment.ident.to_string();
            fmap.insert(ident.clone(), ty);
          }
          _ => unimplemented!(),
        };

        let doc_str = get_docs(&field.attrs);
        typescript.push(format!(
          "{}  {}: {};",
          doc_str,
          ident,
          types_to_ts(&field.ty)
        ));
      }

      metadata.type_defs.insert(name.to_string(), fmap.clone());

      let doc_str = get_docs(&input.attrs);
      let typescript = format!(
        "{}export type {} = {{\n  {}\n}};",
        doc_str,
        name,
        typescript.join("\n")
      );
      metadata.ts_types.insert(name.to_string(), typescript);
      Ok(())
    }
    Data::Enum(syn::DataEnum { variants, .. }) => {
      let name = &input.ident;
      let mut typescript: Vec<String> = vec![];

      for variant in variants {
        let mut variant_fields: Vec<String> = vec![];
        let fields = &variant.fields;
        for field in fields {
          let ident = field
            .ident
            .as_ref()
            .expect("Field without ident")
            .to_string();

          let doc_str = get_docs(&field.attrs);
          variant_fields.push(format!(
            "{}  {}: {};",
            doc_str,
            ident,
            types_to_ts(&field.ty)
          ));
        }

        let doc_str = get_docs(&variant.attrs);
        let variant_str = if variant_fields.len() > 0 {
          format!(
            "{} {{ {}: {{\n {}\n}} }}",
            doc_str,
            &variant.ident,
            variant_fields.join("\n")
          )
        } else {
          format!("{}  \"{}\"", doc_str, &variant.ident)
        };

        typescript.push(variant_str);
      }

      // TODO: `type_defs` in favor of `ts_types`
      metadata.type_defs.insert(name.to_string(), HashMap::new());

      let doc_str = get_docs(&input.attrs);
      let typescript = format!(
        "{}export type {} = {};",
        doc_str,
        name,
        typescript.join("  |\n")
      );
      metadata.ts_types.insert(name.to_string(), typescript);
      Ok(())
    }
    _ => unimplemented!(),
  }
}

fn get_docs(attrs: &Vec<syn::Attribute>) -> String {
  let mut doc: Vec<String> = vec![];
  for attr in attrs {
    if let Ok(syn::Meta::NameValue(meta)) = attr.parse_meta() {
      if !meta.path.is_ident("doc") {
        continue;
      }
      if let syn::Lit::Str(lit) = meta.lit {
        doc.push(lit.value());
      }
    }
  }

  let doc_str = if doc.len() > 0 {
    format!("/**\n  *{}\n  **/\n", doc.join("\n  *"))
  } else {
    String::new()
  };

  doc_str
}

fn types_to_ts(ty: &syn::Type) -> String {
  match ty {
    syn::Type::Array(_) => String::from("any"),
    syn::Type::Ptr(_) => String::from("any"),
    syn::Type::Path(ref ty) => {
      // std::Alloc::Vec => Vec
      let segment = &ty.path.segments.last().unwrap();
      let ty = segment.ident.to_string();
      let mut generics: Vec<String> = vec![];
      let generic_params = &segment.arguments;
      match generic_params {
        &syn::PathArguments::AngleBracketed(ref args) => {
          for p in &args.args {
            let ty = match p {
              syn::GenericArgument::Type(ty) => types_to_ts(ty),
              _ => unimplemented!(),
            };
            generics.push(ty);
          }
        }
        &syn::PathArguments::None => {}
        _ => unimplemented!(),
      };

      match ty.as_ref() {
        "Option" => format!(
          "{} | undefined | null",
          rs_to_ts(generics.first().unwrap().as_ref())
        ),
        _ => {
          if generics.len() > 0 {
            let root_ty = rs_to_ts(&ty);
            let generic_str = generics
              .iter()
              .map(|g| rs_to_ts(g))
              .collect::<Vec<&str>>()
              .join(", ");
            format!("{}<{}>", root_ty, generic_str)
          } else {
            rs_to_ts(&ty).to_string()
          }
        }
      }
    }
    _ => unimplemented!(),
  }
}

fn rs_to_ts(ty: &str) -> &str {
  match ty {
    "i8" => "number",
    "i16" => "number",
    "i32" => "number",
    "i64" => "number",
    "u8" => "number",
    "u16" => "number",
    "u32" => "number",
    "u64" => "number",
    "usize" => "number",
    "bool" => "boolean",
    "String" => "string",
    "f32" => "number",
    "f64" => "number",
    "HashMap" => "Map",
    "Vec" => "Array",
    "HashSet" => "Array",
    "Value" => "any",
    a @ _ => a,
  }
}
