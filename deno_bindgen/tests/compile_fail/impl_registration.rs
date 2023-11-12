use deno_bindgen::deno_bindgen;

// struct Foo is not "registered" in the inventory, so it `impl Foo`
// is not allowed.
struct Foo;

#[deno_bindgen]
impl Foo {
  #[constructor]
  fn new() -> Foo {
    Foo
  }
}

fn main() {}