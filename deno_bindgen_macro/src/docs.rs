// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

use syn::Attribute;
use syn::Lit;
use syn::Meta;

pub fn get_docs(attrs: &Vec<Attribute>) -> String {
  let mut doc: Vec<String> = vec![];
  for attr in attrs {
    if let Ok(Meta::NameValue(meta)) = attr.parse_meta() {
      if !meta.path.is_ident("doc") {
        continue;
      }
      if let Lit::Str(lit) = meta.lit {
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
