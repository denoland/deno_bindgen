// Auto-generated with deno_bindgen

const encode = (s: string) => new TextEncoder().encode(s);
const _lib = Deno.dlopen("target/debug/libadd.so", {
  test_mixed: { parameters: ["isize", "buffer", "usize"], result: "i32" },
  add2: { parameters: ["buffer", "usize"], result: "i32" },
  test_mixed_order: {
    parameters: ["i32", "buffer", "usize", "i32"],
    result: "i32",
  },
  add: { parameters: ["i32", "i32"], result: "i32" },
  test_serde: { parameters: ["buffer", "usize"], result: "u8" },
});
type MyStruct = { arr: any };
type Input = { b: number; a: number };
export function test_mixed(a0: number, a1: Input) {
  const a1_buf = encode(JSON.stringify(a1));
  return _lib.symbols.test_mixed(a0, a1_buf, a1_buf.byteLength) as number;
}
export function add2(a0: Input) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.add2(a0_buf, a0_buf.byteLength) as number;
}
export function test_mixed_order(a0: number, a1: Input, a2: number) {
  const a1_buf = encode(JSON.stringify(a1));
  return _lib.symbols.test_mixed_order(
    a0,
    a1_buf,
    a1_buf.byteLength,
    a2,
  ) as number;
}
export function add(a0: number, a1: number) {
  return _lib.symbols.add(a0, a1) as number;
}
export function test_serde(a0: MyStruct) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.test_serde(a0_buf, a0_buf.byteLength) as number;
}
