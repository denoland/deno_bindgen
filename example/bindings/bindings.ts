// Auto-generated with deno_bindgen
import { Struct, i8, u8, i16, u16, i32le, u32, i64, u64, f32, f64 } from "https://deno.land/x/byte_type/mod.ts";
const usize = u64;
const isize = i64;

const _lib = Deno.dlopen("target/debug/libadd.so", { add: { parameters: [ "i32", "i32" ], result: "i32" }, add2: { parameters: [ "buffer" ], result: "i32" } });
type Input = { a: number; b: number };
const _Input = new Struct({ a: i32le, b: i32le });
export function add(a0: number, a1: number) {
  
  const _result = _lib.symbols.add(a0, a1);
  return _result as number;
}
export function add2(a0: Input) {
  const _buf_a0 = new Uint8Array(_Input.size);
  const _view_a0 = new DataView(_buf_a0.buffer);
  _Input.write(_view_a0, 0, a0);
  const _result = _lib.symbols.add2( _buf_a0);
  return _result as number;
}
  