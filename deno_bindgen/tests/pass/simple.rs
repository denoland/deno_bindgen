use deno_bindgen_macro::deno_bindgen;

#[deno_bindgen]
fn add(a: i32, b: i32) -> i32 {
  a + b
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
struct Foo {
  internal: i32,
}

#[deno_bindgen]
impl Foo {
  #[constructor]
  fn new(internal: i32) -> Foo {
    Foo { internal }
  }

  fn bar(&self) -> i32 {
    42
  }

  fn baz(&self, a: i32) -> i32 {
    a
  }

  fn qux(&self, a: i32, b: i32) -> i32 {
    a + b
  }

  fn quux(&mut self) {
    self.internal += 1;
  }
}

fn main() {}