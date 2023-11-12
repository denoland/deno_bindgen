use deno_bindgen::deno_bindgen;

#[deno_bindgen]
struct Foo;

#[deno_bindgen]
impl Foo {
  #[constructor]
  fn new() -> Foo {
    Foo
  }
}

#[deno_bindgen]
fn foo(_foo: Foo) {} // Fail

#[deno_bindgen]
fn foo2(_foo: &mut Foo) {} // Pass

fn main() {}