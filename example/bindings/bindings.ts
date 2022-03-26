// Auto-generated with deno_bindgen
import { CachePolicy, prepare } from "https://deno.land/x/plug@0.5.1/plug.ts"
function encode(v: string | Uint8Array): Uint8Array {
  if (typeof v !== "string") return v
  return new TextEncoder().encode(v)
}
function decode(v: Uint8Array): string {
  return new TextDecoder().decode(v)
}
function readPointer(v: any): Uint8Array {
  const ptr = new Deno.UnsafePointerView(v as Deno.UnsafePointer)
  const lengthBe = new Uint8Array(4)
  const view = new DataView(lengthBe.buffer)
  ptr.copyInto(lengthBe, 0)
  const buf = new Uint8Array(view.getUint32(0))
  ptr.copyInto(buf, 4)
  return buf
}
const opts = {
  name: "deno_bindgen_test",
  url: (new URL("../target/debug", import.meta.url)).toString(),
  policy: CachePolicy.NONE,
}
const _lib = await prepare(opts, {
  add: { parameters: ["i32", "i32"], result: "i32", nonblocking: false },
  add2: { parameters: ["pointer", "usize"], result: "i32", nonblocking: false },
  sleep: { parameters: ["u64"], result: "void", nonblocking: true },
  test_buf: {
    parameters: ["pointer", "usize"],
    result: "u8",
    nonblocking: false,
  },
  test_buffer_return: {
    parameters: ["pointer", "usize"],
    result: "pointer",
    nonblocking: false,
  },
  test_buffer_return_async: {
    parameters: ["pointer", "usize"],
    result: "pointer",
    nonblocking: true,
  },
  test_hashmap: { parameters: [], result: "pointer", nonblocking: false },
  test_lifetime: {
    parameters: ["pointer", "usize"],
    result: "usize",
    nonblocking: false,
  },
  test_manual_ptr: { parameters: [], result: "pointer", nonblocking: false },
  test_manual_ptr_async: {
    parameters: [],
    result: "pointer",
    nonblocking: true,
  },
  test_mixed: {
    parameters: ["isize", "pointer", "usize"],
    result: "i32",
    nonblocking: false,
  },
  test_mixed_order: {
    parameters: ["i32", "pointer", "usize", "i32"],
    result: "i32",
    nonblocking: false,
  },
  test_mut_buf: {
    parameters: ["pointer", "usize"],
    result: "void",
    nonblocking: false,
  },
  test_output: { parameters: [], result: "pointer", nonblocking: false },
  test_output_async: { parameters: [], result: "pointer", nonblocking: true },
  test_reserved_field: {
    parameters: [],
    result: "pointer",
    nonblocking: false,
  },
  test_serde: {
    parameters: ["pointer", "usize"],
    result: "u8",
    nonblocking: false,
  },
  test_str: {
    parameters: ["pointer", "usize"],
    result: "void",
    nonblocking: false,
  },
  test_str_ret: { parameters: [], result: "pointer", nonblocking: false },
  test_tag_and_content: {
    parameters: ["pointer", "usize"],
    result: "i32",
    nonblocking: false,
  },
})
export type TestLifetimes = {
  text: string
}
/**
 * Doc comment for `Input` struct.
 * ...testing multiline
 */
export type Input = {
  /**
   * Doc comments get
   * transformed to JS doc
   * comments.
   */
  a: number
  b: number
}
export type TagAndContent =
  | { key: "A"; value: { b: number } }
  | { key: "C"; value: { d: number } }
export type WithRecord = {
  my_map: Record<string, string>
}
export type TestReservedField = {
  type: number
  ref: number
}
export type MyStruct = {
  arr: Array<string>
}
export type TestLifetimeWrap = {
  _a: TestLifetimeEnums
}
export type PlainEnum =
  | {
    a: {
      _a: string
    }
  }
  | "b"
  | "c"
export type TestLifetimeEnums = {
  Text: {
    _text: string
  }
}
export type OptionStruct = {
  maybe: string | undefined | null
}
export function add(a0: number, a1: number) {
  let rawResult = _lib.symbols.add(a0, a1)
  const result = rawResult
  return result
}
export function add2(a0: Input) {
  const a0_buf = encode(JSON.stringify(a0))
  let rawResult = _lib.symbols.add2(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function sleep(a0: number) {
  let rawResult = _lib.symbols.sleep(a0)
  const result = rawResult
  return result
}
export function test_buf(a0: Uint8Array) {
  const a0_buf = encode(a0)
  let rawResult = _lib.symbols.test_buf(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function test_buffer_return(a0: Uint8Array) {
  const a0_buf = encode(a0)
  let rawResult = _lib.symbols.test_buffer_return(a0_buf, a0_buf.byteLength)
  const result = readPointer(rawResult)
  return result
}
export function test_buffer_return_async(a0: Uint8Array) {
  const a0_buf = encode(a0)
  let rawResult = _lib.symbols.test_buffer_return_async(
    a0_buf,
    a0_buf.byteLength,
  )
  const result = rawResult.then(readPointer)
  return result
}
export function test_hashmap() {
  let rawResult = _lib.symbols.test_hashmap()
  const result = readPointer(rawResult)
  return JSON.parse(decode(result)) as WithRecord
}
export function test_lifetime(a0: TestLifetimes) {
  const a0_buf = encode(JSON.stringify(a0))
  let rawResult = _lib.symbols.test_lifetime(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function test_manual_ptr() {
  let rawResult = _lib.symbols.test_manual_ptr()
  const result = readPointer(rawResult)
  return result
}
export function test_manual_ptr_async() {
  let rawResult = _lib.symbols.test_manual_ptr_async()
  const result = rawResult.then(readPointer)
  return result
}
export function test_mixed(a0: number, a1: Input) {
  const a1_buf = encode(JSON.stringify(a1))
  let rawResult = _lib.symbols.test_mixed(a0, a1_buf, a1_buf.byteLength)
  const result = rawResult
  return result
}
export function test_mixed_order(a0: number, a1: Input, a2: number) {
  const a1_buf = encode(JSON.stringify(a1))
  let rawResult = _lib.symbols.test_mixed_order(
    a0,
    a1_buf,
    a1_buf.byteLength,
    a2,
  )
  const result = rawResult
  return result
}
export function test_mut_buf(a0: Uint8Array) {
  const a0_buf = encode(a0)
  let rawResult = _lib.symbols.test_mut_buf(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function test_output() {
  let rawResult = _lib.symbols.test_output()
  const result = readPointer(rawResult)
  return JSON.parse(decode(result)) as Input
}
export function test_output_async() {
  let rawResult = _lib.symbols.test_output_async()
  const result = rawResult.then(readPointer)
  return result.then(r => JSON.parse(decode(r))) as Promise<Input>
}
export function test_reserved_field() {
  let rawResult = _lib.symbols.test_reserved_field()
  const result = readPointer(rawResult)
  return JSON.parse(decode(result)) as TestReservedField
}
export function test_serde(a0: MyStruct) {
  const a0_buf = encode(JSON.stringify(a0))
  let rawResult = _lib.symbols.test_serde(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function test_str(a0: string) {
  const a0_buf = encode(a0)
  let rawResult = _lib.symbols.test_str(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function test_str_ret() {
  let rawResult = _lib.symbols.test_str_ret()
  const result = readPointer(rawResult)
  return decode(result)
}
export function test_tag_and_content(a0: TagAndContent) {
  const a0_buf = encode(JSON.stringify(a0))
  let rawResult = _lib.symbols.test_tag_and_content(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
