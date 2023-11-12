#[test]
fn ui_pass() {
  let t = trybuild::TestCases::new();
  t.pass("tests/pass/*.rs");
}

#[test]
fn ui_compile_fail() {
  let t = trybuild::TestCases::new();
  t.compile_fail("tests/compile_fail/*.rs");
}
