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
use syn::Data;
use syn::DataStruct;
use syn::Fields;
use syn::ItemFn;

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

  /// Not-so straightforward types that
  /// `deno_bingen` maps to.
  Struct, // XXX: We need this for
          // transmute "backend".
          // { len: usize }
}

#[derive(Serialize, Deserialize, Clone)]
struct Symbol {
  parameters: Vec<Type>,
  result: Type,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Glue {
  name: String,
  little_endian: bool,
  symbols: HashMap<String, Symbol>,
  type_defs: HashMap<String, HashMap<String, String>>,
}

const METAFILE: &str = "bindings.json";

#[cfg(target_endian = "little")]
const ENDIANNESS: bool = true;

#[cfg(target_endian = "big")]
const ENDIANNESS: bool = false;

#[proc_macro_attribute]
pub fn deno_bindgen(_attr: TokenStream, input: TokenStream) -> TokenStream {
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
      let symbol = process_function(func.clone(), &mut metadata).unwrap();
      (func, symbol)
    }
    Err(_) => {
       let input = syn::parse_macro_input!(input as syn::DeriveInput);
       let fields = process_struct(&mut metadata, input.clone()).unwrap();

       metafile.write_all(&serde_json::to_vec(&metadata).unwrap()).unwrap();
       return TokenStream::from(quote! {
         #[derive(serde::Deserialize)]
         #input
       });
    },
  };

  let mut params = vec![];
  let mut overrides = vec![];
  let mut input_idents = vec![];
  let mut c_index = 0;

  for parameter in symbol.parameters {
    match parameter {
      Type::Struct => {
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
      // TODO
      _ => {
        let ident = format_ident!("arg{}", c_index.to_string());
        params.push(quote! { #ident: i32 });
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
                Type::Struct
              }
            }
          }
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
        _ => panic!("{} return type not supported by Deno FFI", ident),
      }
    }
    _ => unimplemented!(),
  };

  let symbol_name = function.sig.ident.to_string();
  let symbol = Symbol { parameters, result };
  metadata.symbols.insert(symbol_name, symbol.clone());

  Ok(symbol)
}

fn process_struct(
  metadata: &mut Glue,
  input: syn::DeriveInput,
) -> Result<HashMap<String, String>, String> {
  let fields = match &input.data {
    Data::Struct(DataStruct {
      fields: Fields::Named(fields),
      ..
    }) => &fields.named,
    _ => panic!("Expected a struct with named fields"),
  };

  let name = &input.ident;
  let mut fmap = HashMap::new();

  for field in fields.iter() {
    let ident = field.ident.as_ref().expect("Field without ident").to_string();
    match field.ty {
      syn::Type::Path(ref ty) => {
        let segment = &ty.path.segments.first().unwrap();
        let ty = segment.ident.to_string();
        fmap.insert(ident, ty);
      }
      _ => unimplemented!(),
    }
  }
  
  metadata.type_defs.insert(name.to_string(), fmap.clone());
  Ok(fmap)
}
