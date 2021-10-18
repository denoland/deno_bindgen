// Auto-generated with deno_bindgen
import { Plug } from "https://deno.land/x/plug@0.4.0/mod.ts";
const encode = (s: string) => new TextEncoder().encode(s);
const opts = {
  name: "add",
  url: "target/debug",
};
const _lib = await Plug.prepare(opts, {
  add2: { parameters: ["buffer", "usize"], result: "i32" },
  test_mixed_order: {
    parameters: ["i32", "buffer", "usize", "i32"],
    result: "i32",
  },
  test_serde: { parameters: ["buffer", "usize"], result: "u8" },
  add: { parameters: ["i32", "i32"], result: "i32" },
  test_mixed: { parameters: ["isize", "buffer", "usize"], result: "i32" },
});
type MyStruct = {
  arr: Array<string>;
  b: Array<Array<string>>;
};
type Input = {
  a: number;
  b: number;
};
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
export function test_serde(a0: MyStruct) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.test_serde(a0_buf, a0_buf.byteLength) as number;
}
export function add(a0: number, a1: number) {
  return _lib.symbols.add(a0, a1) as number;
}
export function test_mixed(a0: number, a1: Input) {
  const a1_buf = encode(JSON.stringify(a1));
  return _lib.symbols.test_mixed(a0, a1_buf, a1_buf.byteLength) as number;
}
