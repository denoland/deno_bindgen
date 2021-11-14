// Auto-generated with deno_bindgen
import { CachePolicy, prepare } from "https://deno.land/x/plug@0.4.1/plug.ts"
function encode(v: string | Uint8Array): Uint8Array {
  if (typeof v !== "string") return v
  return new TextEncoder().encode(v)
}
const opts = {
  name: "add",
  url: (new URL("../target/debug", import.meta.url)).toString(),
  policy: CachePolicy.NONE,
}
const _lib = await prepare(opts, {
  test_mut_buf: {
    parameters: ["buffer", "usize"],
    result: "void",
    nonblocking: false,
  },
  test_buf: {
    parameters: ["buffer", "usize"],
    result: "u8",
    nonblocking: false,
  },
  add2: { parameters: ["buffer", "usize"], result: "i32", nonblocking: false },
  test_mixed: {
    parameters: ["isize", "buffer", "usize"],
    result: "i32",
    nonblocking: false,
  },
  test_lifetime: {
    parameters: ["buffer", "usize"],
    result: "usize",
    nonblocking: false,
  },
  sleep: { parameters: ["u64"], result: "void", nonblocking: true },
  add: { parameters: ["i32", "i32"], result: "i32", nonblocking: false },
  test_mixed_order: {
    parameters: ["i32", "buffer", "usize", "i32"],
    result: "i32",
    nonblocking: false,
  },
  test_str: {
    parameters: ["buffer", "usize"],
    result: "void",
    nonblocking: false,
  },
  test_serde: {
    parameters: ["buffer", "usize"],
    result: "u8",
    nonblocking: false,
  },
})
export type OptionStruct = {
  maybe: string | undefined | null
}
export type TestLifetimeWrap = {
  a: TestLifetimeEnums
}
export type PlainEnum =
  | {
    a: {
      a: string
    }
  }
  | "b"
  | "c"
export type TestLifetimes = {
  text: string
}
export type TestLifetimeEnums = {
  Text: {
    text: string
  }
}
export type MyStruct = {
  arr: Array<string>
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
export function test_mut_buf(a0: Uint8Array) {
  const a0_buf = encode(a0)
  return _lib.symbols.test_mut_buf(a0_buf, a0_buf.byteLength) as null
}
export function test_buf(a0: Uint8Array) {
  const a0_buf = encode(a0)
  return _lib.symbols.test_buf(a0_buf, a0_buf.byteLength) as number
}
export function add2(a0: Input) {
  const a0_buf = encode(JSON.stringify(a0))
  return _lib.symbols.add2(a0_buf, a0_buf.byteLength) as number
}
export function test_mixed(a0: number, a1: Input) {
  const a1_buf = encode(JSON.stringify(a1))
  return _lib.symbols.test_mixed(a0, a1_buf, a1_buf.byteLength) as number
}
export function test_lifetime(a0: TestLifetimes) {
  const a0_buf = encode(JSON.stringify(a0))
  return _lib.symbols.test_lifetime(a0_buf, a0_buf.byteLength) as number
}
export function sleep(a0: number) {
  return _lib.symbols.sleep(a0) as Promise<null>
}
export function add(a0: number, a1: number) {
  return _lib.symbols.add(a0, a1) as number
}
export function test_mixed_order(a0: number, a1: Input, a2: number) {
  const a1_buf = encode(JSON.stringify(a1))
  return _lib.symbols.test_mixed_order(
    a0,
    a1_buf,
    a1_buf.byteLength,
    a2,
  ) as number
}
export function test_str(a0: string) {
  const a0_buf = encode(a0)
  return _lib.symbols.test_str(a0_buf, a0_buf.byteLength) as null
}
export function test_serde(a0: MyStruct) {
  const a0_buf = encode(JSON.stringify(a0))
  return _lib.symbols.test_serde(a0_buf, a0_buf.byteLength) as number
}
