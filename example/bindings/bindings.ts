// Auto-generated with deno_bindgen
function encode(v: string | Uint8Array): Uint8Array {
  if (typeof v !== "string") return v
  return new TextEncoder().encode(v)
}

function decode(v: Uint8Array): string {
  return new TextDecoder().decode(v)
}

// deno-lint-ignore no-explicit-any
function readPointer(v: any): Uint8Array {
  const ptr = new Deno.UnsafePointerView(v)
  const lengthBe = new Uint8Array(4)
  const view = new DataView(lengthBe.buffer)
  ptr.copyInto(lengthBe, 0)
  const buf = new Uint8Array(view.getUint32(0))
  ptr.copyInto(buf, 4)
  return buf
}

const url = new URL("../target/release", import.meta.url)

import { dlopen, FetchOptions } from "https://deno.land/x/plug@1.0.1/mod.ts"
let uri = url.toString()
if (!uri.endsWith("/")) uri += "/"

let darwin: string | { aarch64: string; x86_64: string } = uri

const opts: FetchOptions = {
  name: "deno_bindgen_test",
  url: {
    darwin,
    windows: uri,
    linux: uri,
  },
  suffixes: {
    darwin: {
      aarch64: "_arm64",
    },
  },
  cache: "use",
}
const { symbols } = await dlopen(opts, {
  add: { parameters: ["i32", "i32"], result: "i32", nonblocking: false },
  add2: { parameters: ["buffer", "usize"], result: "i32", nonblocking: false },
  add3: { parameters: ["f32", "f32"], result: "f32", nonblocking: false },
  add4: { parameters: ["f64", "f64"], result: "f64", nonblocking: false },
  add5: {
    parameters: ["buffer", "usize", "buffer", "usize"],
    result: "buffer",
    nonblocking: false,
  },
  add6: {
    parameters: ["buffer", "usize", "buffer", "usize"],
    result: "buffer",
    nonblocking: false,
  },
  sleep: { parameters: ["u64"], result: "void", nonblocking: true },
  test_buf: {
    parameters: ["buffer", "usize"],
    result: "u8",
    nonblocking: false,
  },
  test_buffer_return: {
    parameters: ["buffer", "usize"],
    result: "buffer",
    nonblocking: false,
  },
  test_buffer_return_async: {
    parameters: ["buffer", "usize"],
    result: "buffer",
    nonblocking: true,
  },
  test_hashmap: { parameters: [], result: "buffer", nonblocking: false },
  test_lifetime: {
    parameters: ["buffer", "usize"],
    result: "usize",
    nonblocking: false,
  },
  test_manual_ptr: { parameters: [], result: "buffer", nonblocking: false },
  test_manual_ptr_async: {
    parameters: [],
    result: "buffer",
    nonblocking: true,
  },
  test_mixed: {
    parameters: ["isize", "buffer", "usize"],
    result: "i32",
    nonblocking: false,
  },
  test_mixed_order: {
    parameters: ["i32", "buffer", "usize", "i32"],
    result: "i32",
    nonblocking: false,
  },
  test_mut_buf: {
    parameters: ["buffer", "usize"],
    result: "void",
    nonblocking: false,
  },
  test_output: { parameters: [], result: "buffer", nonblocking: false },
  test_output_async: { parameters: [], result: "buffer", nonblocking: true },
  test_reserved_field: { parameters: [], result: "buffer", nonblocking: false },
  test_serde: {
    parameters: ["buffer", "usize"],
    result: "u8",
    nonblocking: false,
  },
  test_str: {
    parameters: ["buffer", "usize"],
    result: "void",
    nonblocking: false,
  },
  test_str_ret: { parameters: [], result: "buffer", nonblocking: false },
  test_tag_and_content: {
    parameters: ["buffer", "usize"],
    result: "i32",
    nonblocking: false,
  },
})
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
export type MyStruct = {
  arr: Array<string>
}
export type OptionStruct = {
  maybe: string | undefined | null
}
export type PlainEnum =
  | {
    a: {
      _a: string
    }
  }
  | "b"
  | "c"
export type TagAndContent =
  | { key: "A"; value: { b: number } }
  | { key: "C"; value: { d: number } }
export type TestLifetimeEnums = {
  Text: {
    _text: string
  }
}
export type TestLifetimeWrap = {
  _a: TestLifetimeEnums
}
export type TestLifetimes = {
  text: string
}
export type TestReservedField = {
  type: number
  ref: number
}
export type WithRecord = {
  my_map: Record<string, string>
}
export function add(a0: number, a1: number) {
  const rawResult = symbols.add(a0, a1)
  const result = rawResult
  return result
}
export function add2(a0: Input) {
  const a0_buf = encode(JSON.stringify(a0))

  const rawResult = symbols.add2(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function add3(a0: number, a1: number) {
  const rawResult = symbols.add3(a0, a1)
  const result = rawResult
  return result
}
export function add4(a0: number, a1: number) {
  const rawResult = symbols.add4(a0, a1)
  const result = rawResult
  return result
}
export function add5(a0: Uint8Array, a1: Uint8Array) {
  const a0_buf = encode(a0)
  const a1_buf = encode(a1)

  const rawResult = symbols.add5(
    a0_buf,
    a0_buf.byteLength,
    a1_buf,
    a1_buf.byteLength,
  )
  const result = readPointer(rawResult)
  return result
}
export function add6(a0: Uint8Array, a1: Uint8Array) {
  const a0_buf = encode(a0)
  const a1_buf = encode(a1)

  const rawResult = symbols.add6(
    a0_buf,
    a0_buf.byteLength,
    a1_buf,
    a1_buf.byteLength,
  )
  const result = readPointer(rawResult)
  return result
}
export function sleep(a0: bigint) {
  const rawResult = symbols.sleep(a0)
  const result = rawResult
  return result
}
export function test_buf(a0: Uint8Array) {
  const a0_buf = encode(a0)

  const rawResult = symbols.test_buf(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function test_buffer_return(a0: Uint8Array) {
  const a0_buf = encode(a0)

  const rawResult = symbols.test_buffer_return(a0_buf, a0_buf.byteLength)
  const result = readPointer(rawResult)
  return result
}
export function test_buffer_return_async(a0: Uint8Array) {
  const a0_buf = encode(a0)

  const rawResult = symbols.test_buffer_return_async(a0_buf, a0_buf.byteLength)
  const result = rawResult.then(readPointer)
  return result
}
export function test_hashmap() {
  const rawResult = symbols.test_hashmap()
  const result = readPointer(rawResult)
  return JSON.parse(decode(result)) as WithRecord
}
export function test_lifetime(a0: TestLifetimes) {
  const a0_buf = encode(JSON.stringify(a0))

  const rawResult = symbols.test_lifetime(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function test_manual_ptr() {
  const rawResult = symbols.test_manual_ptr()
  const result = readPointer(rawResult)
  return result
}
export function test_manual_ptr_async() {
  const rawResult = symbols.test_manual_ptr_async()
  const result = rawResult.then(readPointer)
  return result
}
export function test_mixed(a0: bigint, a1: Input) {
  const a1_buf = encode(JSON.stringify(a1))

  const rawResult = symbols.test_mixed(a0, a1_buf, a1_buf.byteLength)
  const result = rawResult
  return result
}
export function test_mixed_order(a0: number, a1: Input, a2: number) {
  const a1_buf = encode(JSON.stringify(a1))

  const rawResult = symbols.test_mixed_order(a0, a1_buf, a1_buf.byteLength, a2)
  const result = rawResult
  return result
}
export function test_mut_buf(a0: Uint8Array) {
  const a0_buf = encode(a0)

  const rawResult = symbols.test_mut_buf(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function test_output() {
  const rawResult = symbols.test_output()
  const result = readPointer(rawResult)
  return JSON.parse(decode(result)) as Input
}
export function test_output_async() {
  const rawResult = symbols.test_output_async()
  const result = rawResult.then(readPointer)
  return result.then(r => JSON.parse(decode(r))) as Promise<Input>
}
export function test_reserved_field() {
  const rawResult = symbols.test_reserved_field()
  const result = readPointer(rawResult)
  return JSON.parse(decode(result)) as TestReservedField
}
export function test_serde(a0: MyStruct) {
  const a0_buf = encode(JSON.stringify(a0))

  const rawResult = symbols.test_serde(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function test_str(a0: string) {
  const a0_buf = encode(a0)

  const rawResult = symbols.test_str(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
export function test_str_ret() {
  const rawResult = symbols.test_str_ret()
  const result = readPointer(rawResult)
  return decode(result)
}
export function test_tag_and_content(a0: TagAndContent) {
  const a0_buf = encode(JSON.stringify(a0))

  const rawResult = symbols.test_tag_and_content(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
