extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use serde_json::json;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Default)]
struct Bindings {
  bindings: Vec<serde_json::Value>,
}

#[proc_macro_attribute]
pub fn deno_bindgen(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(input as syn::ItemFn);
    let func_decl = &func.sig;
    let func_name = &func_decl.ident;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;
    let mut buf = String::new();
    match OpenOptions::new().read(true).open("bindings.json") {
      Ok(mut fd) => {
        fd.read_to_string(&mut buf).unwrap();
      },
      _ => {}
    }

    let dir = Path::new(env!("PROC_ARTIFACT_DIR"));
    let mut f = OpenOptions::new().write(true).create(true).open("bindings.json").unwrap();
    
    let mut bindings: Bindings = serde_json::from_str(&buf).unwrap_or_default();
    let mut bindings_fn = vec![];
    for (idx, i) in func_inputs.iter().enumerate() {
        match i {
            syn::FnArg::Typed(ref val) => match &*val.ty {
                syn::Type::Path(ref ty) => {
                    for seg in &ty.path.segments {
                      let ident = format!("a{}", idx);
                      bindings_fn.push(json!({
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
    bindings.bindings.push(json!({
        "func": func_name.to_string(),
        "parameters": bindings_fn
      }
    ));
    f.write_all(&serde_json::to_vec(&bindings).unwrap()).unwrap();
    TokenStream::from(quote! {
      #[no_mangle]
      pub extern "C" #func
    })
}
