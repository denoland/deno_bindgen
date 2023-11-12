use deno_bindgen::deno_bindgen;

#[deno_bindgen]
struct Input {
  a: i32,
  b: i32,
}

#[deno_bindgen]
impl Input {
  #[constructor]
  fn new(a: i32, b: i32) -> Input {
    Input { a, b }
  }

  #[constructor]
  fn new2() -> Input {
    Input { a: 0, b: 0 }
  }
}

fn main() {}