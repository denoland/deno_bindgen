use syn::Attribute;
use syn::Lit;
use syn::Meta;
use syn::NestedMeta;

use inflector::Inflector;

#[derive(Debug)]
pub enum SerdeAttr {
  RenameAll(String),
}

impl SerdeAttr {
  pub fn transform(&self, s: &str) -> String {
    match self {
      SerdeAttr::RenameAll(t) => match t.as_ref() {
        "lowercase" => s.to_lowercase(),
        "UPPERCASE" => s.to_uppercase(),
        "camelCase" => s.to_camel_case(),
        "snake_case" => s.to_snake_case(),
        "PascalCase" => s.to_pascal_case(),
        "SCREAMING_SNAKE_CASE" => s.to_screaming_snake_case(),
        _ => panic!("Invalid attribute value: {}", s),
      },
    }
  }
}

pub fn get_serde_attrs(attrs: &Vec<Attribute>) -> Vec<SerdeAttr> {
  attrs
    .iter()
    .filter(|i| i.path.is_ident("serde"))
    .flat_map(|attr| match attr.parse_meta() {
      Ok(Meta::List(l)) => l.nested.iter().find_map(|meta| match meta {
        NestedMeta::Meta(Meta::NameValue(v)) => {
          if v.path.is_ident("rename_all") {
            match &v.lit {
              Lit::Str(s) => Some(SerdeAttr::RenameAll(s.value())),
              _ => None,
            }
          } else {
            None
          }
        }
        _ => None,
      }),
      _ => None,
    })
    .collect::<Vec<_>>()
}
