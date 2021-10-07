use deno_bindgen::deno_bindgen;

#[deno_bindgen]
fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[deno_bindgen]
pub struct Input {
  a: i32,
  b: i32,
}

#[deno_bindgen]
fn add2(input: Input) -> i32 {
  input.a + input.b
}
