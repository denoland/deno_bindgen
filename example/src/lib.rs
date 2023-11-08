use deno_bindgen::deno_bindgen;

#[deno_bindgen]
fn add(a: i32, b: i32) -> i32 {
  a + b
}

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
}

#[deno_bindgen]
fn add2(input: &Input) -> i32 {
  input.a + input.b
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
fn make_foo() -> Foo {
  Foo { internal: 42 }
}

#[deno_bindgen]
fn inc_foo(foo: &mut Foo) {
  foo.internal += 1;
}

#[deno_bindgen]
pub struct Foo {
  internal: u32,
}

#[deno_bindgen]
impl Foo {
  #[constructor]
  fn new(internal: u32) -> Foo {
    Foo { internal }
  }
  
  fn inc(&mut self) {
    self.internal += 1;
  }

  fn bar(&self, a: u32) -> u32 {
    self.internal + a
  }
}