#![feature(box_patterns)]

use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use syn::Data;
use syn::DataStruct;
use syn::Fields;
use syn::ItemFn;

#[derive(Serialize, Deserialize, Default)]
struct Bindings {
  name: String,
  bindings: Vec<serde_json::Value>,
  type_defs: Vec<serde_json::Value>,
}

#[proc_macro_attribute]
pub fn deno_bindgen(_attr: TokenStream, input: TokenStream) -> TokenStream {
  let mut buf = String::new();
  // Load existing bindings
  match OpenOptions::new().read(true).open("bindings.json") {
    Ok(mut fd) => {
      fd.read_to_string(&mut buf).unwrap();
    }
    _ => {
      // We assume this was the first macro run.
    }
  }
  let mut bindings: Bindings = serde_json::from_str(&buf).unwrap_or_default();
  // TODO(@littledivy): Use Cargo's `out` directory
  // let dir = Path::new(env!("PROC_ARTIFACT_DIR"));
  let mut config = OpenOptions::new()
    .write(true)
    .create(true)
    .open("bindings.json")
    .unwrap();

  let pkg_name = std::env::var("CARGO_CRATE_NAME").unwrap();

  match syn::parse::<ItemFn>(input.clone()) {
    //
    Ok(func) => {
      let mut parameters = vec![];

      let fn_name = &func.sig.ident;
      let fn_inputs = &func.sig.inputs;
      let fn_output = &func.sig.output;
      let fn_block = &func.block;
      let fn_params: Vec<_> = fn_inputs
        .iter()
        .enumerate()
        .map(|(idx, i)| match i {
          syn::FnArg::Typed(ref val) => {
            match &*val.ty {
              syn::Type::Path(ref ty) => {
                for seg in &ty.path.segments {
                  let ident = format!("a{}", idx);
                  parameters.push(json!(
                    {
                      "ident": ident,
                      "type": type_identifier(&bindings, &seg.ident.to_string()),
                    }
                  ));
                }
              }
              _ => {}
            };

            &val.pat
          }
          _ => unimplemented!(),
        })
        .collect();

      let return_type = match &func.sig.output {
        syn::ReturnType::Default => "void".to_string(),
        syn::ReturnType::Type(_, box syn::Type::Path(ty)) => {
          // TODO(@littledivy): Support multiple `Type` path segments
          ty.path.segments[0].ident.to_string()
        }
        _ => unimplemented!(),
      };

      bindings.bindings.push(json!(
        {
          "func": func.sig.ident.to_string(),
          "parameters": parameters,
          "result": return_type,
        }
      ));

      bindings.name = pkg_name.to_string();
      config
        .write_all(&serde_json::to_vec(&bindings).unwrap())
        .unwrap();

      TokenStream::from(quote! {
        #[no_mangle]
        pub extern "C" fn #fn_name (#fn_inputs) #fn_output {
          fn __inner_impl (#fn_inputs) #fn_output #fn_block
          let result = __inner_impl(#(#fn_params,) *);
          result
        }
      })
    }
    Err(_) => {
      // Try to parse as an DeriveInput
      let input = syn::parse_macro_input!(input as syn::DeriveInput);
      let fields = match &input.data {
        Data::Struct(DataStruct {
          fields: Fields::Named(fields),
          ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
      };
      let struct_name = &input.ident;
      let mut definition = json!({});

      for field in fields.iter() {
        if let Some(ident) = &field.ident {
          match field.ty {
            syn::Type::Path(ref ty) => {
              for seg in &ty.path.segments {
                definition[ident.to_string()] =
                  serde_json::Value::String(seg.ident.to_string());
              }
            }
            _ => {}
          }
        }
      }

      bindings.type_defs.push(
        json!({ "ident": struct_name.to_string(), "fields": definition }),
      );
      config
        .write_all(&serde_json::to_vec(&bindings).unwrap())
        .unwrap();
      TokenStream::from(quote! {
        // Preserve the input
        #[repr(C)]
        #input
      })
    }
  }
}

fn type_identifier(bindings: &Bindings, ty: &str) -> String {
  match ty {
    "void" | "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64"
    | "usize" | "isize" | "f32" | "f64" => ty.to_string(),
    _ => {
      // Check if a type definition already exists
      bindings
        .type_defs
        .iter()
        .find(|&def| def["ident"] == ty)
        .expect(&format!("Type definition not found for `{}` identifier", ty));
      ty.to_string()
    }
  }
}
 