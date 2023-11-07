use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_quote, Pat};

pub mod codegen;
pub mod inventory;

pub use inventory::Inventory;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum Type {
  #[default]
  Void,
  Uint8,
  Uint16,
  Uint32,
  Uint64,
  Int8,
  Int16,
  Int32,
  Int64,
  Float32,
  Float64,
  Pointer,
  Buffer,
}

pub type RawTypes = &'static [Type];

impl Type {
  pub fn raw(&self) -> RawTypes {
    match self {
      Self::Buffer => &[Self::Pointer, Self::Uint32],
      Self::Pointer => &[Self::Pointer],
      _ => &[],
    }
  }

  pub fn is_number(&self) -> bool {
    !matches!(self, Self::Void | Self::Pointer | Self::Buffer)
  }

  pub fn apply_transform(
    &self,
    name: &mut Box<Pat>,
    args: &[Ident],
  ) -> Option<proc_macro2::TokenStream> {
    match self {
      Self::Buffer => {
        let pointer = &args[0];
        let length = &args[1];
        Some(quote! {
          let #name = unsafe {
            std::slice::from_raw_parts_mut(#pointer as _, #length as usize)
          };
        })
      }
      Self::Pointer => {
        let pointer = &args[0];
        Some(quote! {
          let #name = #pointer as _;
        })
      }
      _ => None,
    }
  }

  pub fn to_ident(&self) -> syn::Type {
    match self {
      Self::Void => parse_quote!(deno_bindgen::Type::Void),
      Self::Uint8 => parse_quote!(deno_bindgen::Type::Uint8),
      Self::Uint16 => parse_quote!(deno_bindgen::Type::Uint16),
      Self::Uint32 => parse_quote!(deno_bindgen::Type::Uint32),
      Self::Uint64 => parse_quote!(deno_bindgen::Type::Uint64),
      Self::Int8 => parse_quote!(deno_bindgen::Type::Int8),
      Self::Int16 => parse_quote!(deno_bindgen::Type::Int16),
      Self::Int32 => parse_quote!(deno_bindgen::Type::Int32),
      Self::Int64 => parse_quote!(deno_bindgen::Type::Int64),
      Self::Float32 => parse_quote!(deno_bindgen::Type::Float32),
      Self::Float64 => parse_quote!(deno_bindgen::Type::Float64),
      Self::Pointer => parse_quote!(deno_bindgen::Type::Pointer),
      Self::Buffer => parse_quote!(deno_bindgen::Type::Buffer),
    }
  }
}

impl ToTokens for Type {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let ty = match self {
      Self::Void => quote! { () },
      Self::Uint8 => quote! { u8 },
      Self::Uint16 => quote! { u16 },
      Self::Uint32 => quote! { u32 },
      Self::Uint64 => quote! { u64 },
      Self::Int8 => quote! { i8 },
      Self::Int16 => quote! { i16 },
      Self::Int32 => quote! { i32 },
      Self::Int64 => quote! { i64 },
      Self::Float32 => quote! { f32 },
      Self::Float64 => quote! { f64 },
      Self::Pointer => quote! { *const () },
      Self::Buffer => quote! { *mut u8 },
    };

    tokens.extend(ty);
  }
}

#[derive(Debug)]
pub struct Symbol {
  pub name: &'static str,
  pub parameters: &'static [Type],
  pub return_type: Type,
  pub non_blocking: bool,
}

pub struct SymbolBuilder {
  name: Ident,
  parameters: Vec<Type>,
  return_type: Type,
  non_blocking: bool,
}

impl SymbolBuilder {
  pub fn new(name: Ident) -> Self {
    Self {
      name,
      parameters: Vec::new(),
      return_type: Default::default(),
      non_blocking: false,
    }
  }

  pub fn push(&mut self, ty: Type) {
    self.parameters.push(ty);
  }

  pub fn return_type(&mut self, ty: Type) {
    self.return_type = ty;
  }

  pub fn non_blocking(&mut self, non_blocking: bool) {
    self.non_blocking = non_blocking;
  }
}

impl ToTokens for SymbolBuilder {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let parameters = &self
      .parameters
      .iter()
      .map(|ty| ty.to_ident())
      .collect::<Vec<_>>();
    let return_type = &self.return_type.to_ident();
    let non_blocking = &self.non_blocking;
    let name = &self.name;

    tokens.extend(quote! {
       deno_bindgen::Symbol {
          name: stringify!(#name),
          parameters: &[#(#parameters),*],
          return_type: #return_type,
          non_blocking: #non_blocking,
       }
    });
  }
}
