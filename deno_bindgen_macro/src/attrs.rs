// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

use syn::Attribute;
use syn::Lit;
use syn::Meta;
use syn::NestedMeta;

use inflector::Inflector;

#[derive(Debug)]
pub enum SerdeAttr {
  RenameAll(String),
  TagAndContent(String, String),
}

impl SerdeAttr {
  pub fn transform(&self, s: &str) -> Option<String> {
    match self {
      SerdeAttr::RenameAll(t) => match t.as_ref() {
        "lowercase" => Some(s.to_lowercase()),
        "UPPERCASE" => Some(s.to_uppercase()),
        "camelCase" => Some(s.to_camel_case()),
        "snake_case" => Some(s.to_snake_case()),
        "PascalCase" => Some(s.to_pascal_case()),
        "SCREAMING_SNAKE_CASE" => Some(s.to_screaming_snake_case()),
        _ => panic!("Invalid attribute value: {}", s),
      },
      _ => None,
    }
  }
}

pub fn get_serde_attrs(attrs: &Vec<Attribute>) -> Vec<SerdeAttr> {
  attrs
    .iter()
    .filter(|i| i.path.is_ident("serde"))
    .flat_map(|attr| match attr.parse_meta() {
      Ok(Meta::List(l)) => l.nested.iter().find_map(|meta| match meta {
        NestedMeta::Meta(Meta::NameValue(v)) => match v.path.get_ident() {
          Some(id) => match id.to_string().as_ref() {
            // #[serde(rename_all = "UPPERCASE")]
            "rename_all" => match &v.lit {
              Lit::Str(s) => Some(SerdeAttr::RenameAll(s.value())),
              _ => None,
            },
            // #[serde(tag = "key", content = "value")]
            "tag" => match &v.lit {
              Lit::Str(s) => {
                let tag = s.value();

                let lit = l.nested.iter().find_map(|meta| match meta {
                  NestedMeta::Meta(Meta::NameValue(v)) => {
                    match v.path.is_ident("content") {
                      true => Some(&v.lit),
                      false => None,
                    }
                  }
                  _ => None,
                });

                match lit {
                  Some(Lit::Str(s)) => {
                    let content = s.value();
                    Some(SerdeAttr::TagAndContent(tag, content))
                  }
                  _ => panic!("Missing `content` attribute with `tag`."),
                }
              }
              _ => None,
            },
            // #[serde(content = "value", tag = "key")]
            "content" => match &v.lit {
              Lit::Str(s) => {
                let content = s.value();

                let lit = l.nested.iter().find_map(|meta| match meta {
                  NestedMeta::Meta(Meta::NameValue(v)) => {
                    match v.path.is_ident("tag") {
                      true => Some(&v.lit),
                      false => None,
                    }
                  }
                  _ => None,
                });

                match lit {
                  Some(Lit::Str(s)) => {
                    let tag = s.value();
                    Some(SerdeAttr::TagAndContent(tag, content))
                  }
                  _ => panic!("Missing `tag` attribute with `content`."),
                }
              }
              _ => None,
            },
            _ => None,
          },
          _ => None,
        },
        _ => None,
      }),
      _ => None,
    })
    .collect::<Vec<_>>()
}
