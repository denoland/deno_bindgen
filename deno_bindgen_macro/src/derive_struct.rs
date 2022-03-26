// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

use crate::attrs::get_serde_attrs;
use crate::attrs::SerdeAttr;
use crate::docs::get_docs;
use crate::meta::Glue;

use std::collections::HashMap;
use syn::ext::IdentExt;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Fields;

macro_rules! variant_instance {
  ( $variant:path, $iterator:expr ) => {
    $iterator
      .filter_map(|val| {
        if let $variant(ref f1, ref f2) = *val {
          Some((f1, f2))
        } else {
          None
        }
      })
      .next()
  };
}

pub fn process_struct(
  metadata: &mut Glue,
  input: DeriveInput,
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

      let serde_attrs = get_serde_attrs(&input.attrs);

      for field in fields.iter() {
        let mut ident = field
          .ident
          .as_ref()
          .expect("Field without ident")
          // Strips the raw marker `r#`, if present.
          .unraw()
          .to_string();

        match field.ty {
          syn::Type::Path(ref ty) => {
            let segment = &ty.path.segments.first().unwrap();
            let ty = segment.ident.to_string();
            fmap.insert(ident.clone(), ty);
          }
          syn::Type::Reference(ref ty) => {
            assert!(!ty.mutability.is_some());
            assert!(ty.lifetime.is_some());
            match *ty.elem {
              syn::Type::Path(ref ty) => {
                let segment = &ty.path.segments.first().unwrap();
                let ty = segment.ident.to_string();
                fmap.insert(ident.clone(), ty);
              }
              _ => unimplemented!(),
            }
          }
          _ => unimplemented!(),
        };

        for attr in &serde_attrs {
          if let Some(i) = attr.transform(&ident) {
            ident = i;
          }
        }

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

        let serde_attrs = get_serde_attrs(&input.attrs);
        for field in fields {
          let mut ident = field
            .ident
            .as_ref()
            .expect("Field without ident")
            // Strips the raw marker `r#`, if present.
            .unraw()
            .to_string();

          for attr in &serde_attrs {
            if let Some(i) = attr.transform(&ident) {
              ident = i;
            }
          }

          let doc_str = get_docs(&field.attrs);
          variant_fields.push(format!(
            "{}  {}: {};",
            doc_str,
            ident,
            types_to_ts(&field.ty)
          ));
        }

        let mut ident = variant.ident.to_string();

        // Perform #[serde] attribute transformers.
        // This excludes `tag` and `content` attributes.
        // They require special treatment during codegen.
        for attr in &serde_attrs {
          if let Some(i) = attr.transform(&ident) {
            ident = i;
          }
        }

        let doc_str = get_docs(&variant.attrs);

        let variant_str = if variant_fields.len() > 0 {
          let tag_content =
            variant_instance!(SerdeAttr::TagAndContent, serde_attrs.iter());

          match tag_content {
            None => {
              format!(
                "{} {{ {}: {{\n {}\n}} }}",
                doc_str,
                &ident,
                variant_fields.join("\n")
              )
            }
            Some((tag, content)) => {
              // // $jsdoc
              // {
              //   $tag: $ident,
              //   $content: { ...$fields }
              // }
              format!(
                "{} {{ {}: \"{}\", {}: {{ {} }} }}",
                doc_str,
                &tag,
                &ident,
                &content,
                variant_fields.join("\n")
              )
            }
          }
        } else {
          format!("{}  \"{}\"", doc_str, &ident)
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

fn types_to_ts(ty: &syn::Type) -> String {
  match ty {
    syn::Type::Array(_) => String::from("any"),
    syn::Type::Ptr(_) => String::from("any"),
    syn::Type::Reference(ref ty) => types_to_ts(&ty.elem),
    syn::Type::Path(ref ty) => {
      // std::alloc::Vec => Vec
      let segment = &ty.path.segments.last().unwrap();
      let ty = segment.ident.to_string();
      let mut generics: Vec<String> = vec![];
      let generic_params = &segment.arguments;
      match generic_params {
        &syn::PathArguments::AngleBracketed(ref args) => {
          for p in &args.args {
            let ty = match p {
              syn::GenericArgument::Type(ty) => types_to_ts(ty),
              syn::GenericArgument::Lifetime(_) => continue,
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
    "str" => "string",
    "f32" => "number",
    "f64" => "number",
    "HashMap" => "Record",
    "Vec" => "Array",
    "HashSet" => "Array",
    "Value" => "any",
    a @ _ => a,
  }
}
