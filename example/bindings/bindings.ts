// Auto-generated with deno_bindgen
import { Struct, i8, u8, i16le, u16le, i32le, u32le, i64le, u64le, f32le, f64le } from "https://deno.land/x/byte_type/mod.ts";
const i16 = i16le;
const u16 = u16le;
const i32 = i32le;
const u32 = u32le;
const i64 = i64le;
const u64 = u64le;
const f32 = f32le;
const f64 = f64le;
const usize = u64;
const isize = i64;

const _lib = Deno.dlopen("target/debug/libadd.so", { add: { parameters: [ "i32", "i32" ], result: "i32" }, add2: { parameters: [ "buffer" ], result: "i32" }, print_buf: { parameters: [  ], result: "void" } });
type Input = { a: number; b: number };
const _Input = new Struct({ a: i32, b: i32 });
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
export function print_buf() {
  
  const _result = _lib.symbols.print_buf();
  return _result as null;
}
  