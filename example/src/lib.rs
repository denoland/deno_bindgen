use deno_bindgen::deno_bindgen;
use std::collections::HashMap;

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
  #[allow(dead_code)]
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
fn test_str(_s: &str) {}

#[deno_bindgen]
fn test_buf(b: &[u8]) -> u8 {
  b[0]
}

#[deno_bindgen]
#[serde(rename_all = "lowercase")]
enum PlainEnum {
  A { _a: String },
  B,
  C,
}

// Test mut buffer
#[deno_bindgen]
fn test_mut_buf(buf: &mut [u8]) {
    buf[0] = 69;
}

// Test mut buffer prevent return
// #[deno_bindgen]
// fn test_mut_buf_ret(buf: &mut [u8]) -> &mut [u8] {
//   buf
// }

// Test mut buffer musn't outlive symbol call
// #[deno_bindgen]
// fn test_mut_buf_outlive(_: &'static mut [u8]) {
//  
// }

#[deno_bindgen]
struct TestLifetimes<'l> {
  text: &'l str,
}

#[deno_bindgen]
enum TestLifetimeEnums<'a> {
  Text { _text: &'a str }
}

#[deno_bindgen]
struct TestLifetimeWrap<'a> {
  #[serde(borrow)]
  _a: TestLifetimeEnums<'a>
}

#[deno_bindgen]
fn test_lifetime<'l>(s: TestLifetimes<'l>) -> usize {
  s.text.len()
}

#[deno_bindgen]
#[serde(tag = "key", content = "value")]
pub enum TagAndContent {
  A { b: i32 },
  C { d: i32 }
}


#[deno_bindgen]
fn test_tag_and_content(arg: TagAndContent) -> i32 {
  if let TagAndContent::A { b } = arg {
    b
  } else {
    -1
  }
}

#[deno_bindgen]
fn test_buffer_return(buf: &[u8]) -> &[u8] {
  buf
}

#[deno_bindgen(non_blocking)]
fn test_buffer_return_async(buf: &[u8]) -> &[u8] {
  buf
}

#[deno_bindgen]
fn test_manual_ptr() -> *const u8 {
  let result = String::from("test").into_bytes();
  let length = (result.len() as u32).to_be_bytes();
  let mut v = length.to_vec();
  v.extend(result.clone());

  let ret = v.as_ptr();
  // Leak the result to JS land.
  ::std::mem::forget(v);
  ret
}

#[deno_bindgen(non_blocking)]
fn test_manual_ptr_async() -> *const u8 {
  let result = String::from("test").into_bytes();
  let length = (result.len() as u32).to_be_bytes();
  let mut v = length.to_vec();
  v.extend(result.clone());

  let ret = v.as_ptr();
  // Leak the result to JS land.
  ::std::mem::forget(v);
  ret
}

#[deno_bindgen]
fn test_output() -> Input {
  Input {
    a: 1,
    b: 2
  }
}

#[deno_bindgen(non_blocking)]
fn test_output_async() -> Input {
  Input {
    a: 3,
    b: 4
  }
}

#[deno_bindgen]
struct TestReservedField {
  r#type: u8,
  r#ref: u8,
}

#[deno_bindgen]
fn test_reserved_field() -> TestReservedField {
  TestReservedField {
    r#type: 1,
    r#ref: 2,
  }
}

#[deno_bindgen]
fn test_str_ret() -> String {
  String::from("🦕")
} 

#[deno_bindgen]
pub struct WithRecord {
  my_map: HashMap<String, String>,
}

#[deno_bindgen]
fn test_hashmap() -> WithRecord {
  let mut map = HashMap::new();
  map.insert("key".to_string(), "value".to_string());
  WithRecord {
    my_map: map,
  }
}
