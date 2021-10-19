use deno_bindgen::deno_bindgen;

// Test "primitives"
#[deno_bindgen]
fn add(a: i32, b: i32) -> i32 {
  a + b
}

// Test Structs
#[deno_bindgen]
/// Doc comment for `Input` struct.
/// ...testing multiline
pub struct Input {
  /// Doc comments get
  /// transformed to JS doc
  /// comments.
  a: i32,
  b: i32,
}

#[deno_bindgen]
fn add2(input: Input) -> i32 {
  input.a + input.b
}

// Test mixed types
#[deno_bindgen]
fn test_mixed(a: isize, b: Input) -> i32 {
  a as i32 + b.a
}

// Test mixed type codegen order
#[deno_bindgen]
fn test_mixed_order(a: i32, b: Input, c: i32) -> i32 {
  a + b.a + c
}

// Test serde support
#[deno_bindgen]
struct MyStruct {
  arr: Vec<String>,
}

#[deno_bindgen]
fn test_serde(s: MyStruct) -> u8 {
  if s.arr.contains(&"WORKS".to_string()) {
    return 1;
  }
  0
}

// Typescript codegen tests

#[deno_bindgen]
struct OptionStruct {
  maybe: Option<String>,
}

// Test non_blocking
#[deno_bindgen(non_blocking)]
fn sleep(ms: u64) {
  std::thread::sleep(std::time::Duration::from_millis(ms));
}

// Test other buffer dependent
// types.
#[deno_bindgen]
fn test_str(s: &str) {}

#[deno_bindgen]
fn test_buf(b: &[u8]) -> u8 {
  b[0]
}
