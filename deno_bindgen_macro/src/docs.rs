// Copyright 2020-2023 the Deno authors. All rights reserved. MIT license.

use syn::{Attribute, Lit, Meta};

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
