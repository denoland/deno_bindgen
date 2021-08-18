use deno_bindgen::deno_bindgen;

#[deno_bindgen]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[deno_bindgen]
fn add2(a: i32, b: i32) -> i32 {
    a + b
}
#[test]
fn normal() {
    assert_eq!(add(1, 2), 3);
}
