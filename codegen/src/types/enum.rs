use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use inflector::Inflector;

use super::NativeType;
use super::TypeConverter;
use super::TypeDefinition;
use super::TypeDescriptor;

fn hashed_variants_identifer(
  fields: &[(String, Option<TypeDefinition>)],
) -> String {
  let mut hasher = DefaultHasher::new();
  fields.hash(&mut hasher);
  format!("{:x}", hasher.finish())
}

#[derive(Clone, Hash)]
pub enum EnumDiscriminantType {
  U8,
  I8,
  U16,
  I16,
  U32,
  I32,
  U64,
  I64,
  USize,
  ISize,
}

#[derive(Clone, Hash)]
pub struct Enum {
  pub identifier: String,
  pub anonymous: bool,
  pub discriminant: EnumDiscriminantType,
  pub variants: Vec<(String, Option<TypeDefinition>)>,
}

impl Enum {
  pub fn new(
    identifier: Option<&str>,
    discriminant: EnumDiscriminantType,
    variants: Vec<(String, Option<TypeDefinition>)>,
  ) -> Self {
    Self {
      identifier: identifier
        .map(String::from)
        .unwrap_or_else(|| hashed_variants_identifer(&variants)),
      anonymous: identifier.is_none(),
      discriminant,
      variants,
    }
  }

  pub fn typescript_type(&self) -> String {
    self
      .variants()
      .iter()
      .map(|(property, r#type)| {
        if let Some((definition, descriptor)) = r#type {
          format!(
            "{{ tag: {}, value: {} }}",
            property, descriptor.converter.typescript
          )
        } else {
          format!("{{ tag: {} }}", property)
        }
      })
      .collect::<Vec<String>>()
      .join(" | ")
  }

  pub fn variants(
    &self,
  ) -> Vec<(String, Option<(TypeDefinition, TypeDescriptor)>)> {
    self
      .variants
      .clone()
      .into_iter()
      .map(|(property, definition)| {
        (
          property,
          definition.map(|definition| {
            (definition.clone(), TypeDescriptor::from(definition))
          }),
        )
      })
      .collect()
  }

  pub fn typescript(&self) -> String {
    if self.anonymous {
      self.typescript_type()
    } else {
      self.identifier.to_pascal_case()
    }
  }

  pub fn into_function_name(&self) -> String {
    format!("__into_{}", self.identifier)
  }

  pub fn from_function_name(&self) -> String {
    format!("__from_{}", self.identifier)
  }

  pub fn generate_into_function(&self, globals: &mut Vec<String>) {
    todo!()
  }

  pub fn generate_from_function(&self, globals: &mut Vec<String>) {
    todo!()
  }
}

impl From<Enum> for TypeDescriptor {
  fn from(r#enum: Enum) -> Self {
    let mut globals = Vec::new();

    if !r#enum.anonymous {
      globals.push(format!(
        "export type {} = {}",
        r#enum.typescript(),
        r#enum.typescript_type()
      ));
    }

    r#enum.generate_into_function(&mut globals);
    r#enum.generate_from_function(&mut globals);

    TypeDescriptor {
      native: NativeType::Pointer,
      converter: TypeConverter {
        globals,
        typescript: r#enum.typescript(),
        into: format!("{}({{}})", r#enum.into_function_name()),
        from: format!("{}({{}})", r#enum.from_function_name()),
      },
    }
  }
}
