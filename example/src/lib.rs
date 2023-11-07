use deno_bindgen::deno_bindgen;

// Test "primitives"
#[deno_bindgen]
fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[deno_bindgen]
fn add2(a: i32, b: i32) -> i32 {
  a + b
}

#[deno_bindgen]
fn bytelen(b: &[u8]) -> u32 {
  b.len() as u32
}

#[deno_bindgen]
fn buf_mut(b: &mut [u8]) {
  b[0] = 99;
}

#[deno_bindgen]
fn cstr() -> *const u8 {
  b"Hello, World!\0".as_ptr()
}

#[deno_bindgen]
fn strlen(s: *const u8) -> u32 {
  let mut len = 0;
  unsafe {
    while *s.add(len as usize) != 0 {
      len += 1;
    }
  }
  len
}

#[deno_bindgen(non_blocking)]
fn non_blocking() -> i32 {
  42
}

#[deno_bindgen]
pub struct Foo {
  internal: i32,
}

#[deno_bindgen]
impl Foo {
  fn new() -> Self {
    Self { internal: 42 }
  }
}