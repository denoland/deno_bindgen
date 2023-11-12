use crate::Symbol;

#[derive(Debug)]
pub struct Struct {
  pub name: &'static str,
  pub methods: &'static [Symbol],
}

pub enum Inventory {
  Symbol(Symbol),
  Struct(Struct),
}
