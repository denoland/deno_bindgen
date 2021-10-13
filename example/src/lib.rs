use deno_bindgen::deno_bindgen;

// Test "primitives"

#[deno_bindgen]
fn add(a: i32, b: i32) -> i32 {
  a + b
}
 
// Test Structs

#[deno_bindgen]
pub struct Input {
  a: i32,
  b: i32,
}

#[deno_bindgen]
fn add2(input: Input) -> i32 {
  input.a + input.b
}

// Test mixed types

#[deno_bindgen]
fn test_mixed(a: i32, b: Input) -> i32 {
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
fn test_serde(s: MyStruct) -> i32 {
    if s.arr.contains(&"WORKS".to_string()) {
        return 1;
    } 
    0
}

