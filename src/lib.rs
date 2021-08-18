#![feature(box_patterns)]

use proc_macro::TokenStream;
use quote::quote;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Default)]
struct Bindings {
  name: String,
  bindings: Vec<serde_json::Value>,
}

#[proc_macro_attribute]
pub fn deno_bindgen(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(input as syn::ItemFn);    
    let mut buf = String::new();
    // Load existing bindings
    match OpenOptions::new().read(true).open("bindings.json") {
      Ok(mut fd) => {
        fd.read_to_string(&mut buf).unwrap();
      },
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
    
    let mut parameters = vec![];
    let pkg_name = env!("CARGO_PKG_NAME");
    for (idx, i) in func.sig.inputs.iter().enumerate() {
        match i {
            syn::FnArg::Typed(ref val) => match &*val.ty {
                syn::Type::Path(ref ty) => {
                    for seg in &ty.path.segments {
                      let ident = format!("a{}", idx);
                      parameters.push(json!({
                        "ident": ident,
                        "type": seg.ident.to_string(),
                      }));
                    }
                }
                _ => {}
            },
            _ => unreachable!(),
        }
    }

    let return_type = match &func.sig.output {
      syn::ReturnType::Default => "void".to_string(),
      syn::ReturnType::Type(_, box syn::Type::Path(ty)) => {
        // TODO(@littledivy): Support multiple `Type` path segments
        ty.path.segments[0].ident.to_string()
      }
      _ => panic!("Type not supported"),
    };

    bindings.bindings.push(json!({
        "func": func.sig.ident.to_string(),
        "parameters": parameters,
        "result": return_type,
      }
    ));
    bindings.name = pkg_name.to_string();
    config.write_all(&serde_json::to_vec(&bindings).unwrap()).unwrap();

    TokenStream::from(quote! {
      #[no_mangle]
      pub extern "C" #func
    })
}
