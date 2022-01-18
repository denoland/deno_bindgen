import * as Plug from "https://deno.land/x/plug/mod.ts";
export const library = await Plug.prepare({ name: "test", url: "abcdef" }, {
  test: { parameters: ["usize"], result: "pointer", nonblocking: false },
});
const __cstring_encoder = new TextEncoder();
export interface ExampleInnerStruct {
  inner_a: Deno.UnsafePointer;
  inner_b: bigint;
}
function __into_ExampleInnerStruct(__data: ExampleInnerStruct): Uint8Array {
  const __array_buffer = new ArrayBuffer(16);
  const __u8_view = new Uint8Array(__array_buffer);
  const __u64_view = new BigUint64Array(__array_buffer);
  const __i64_view = new BigInt64Array(__array_buffer);
  __u64_view[0] = __data.inner_a.value;
  __i64_view[1] = __data.inner_b;
  return __u8_view;
}
function __into_63131302a3ae1784(__data: {
  a: number;
  b: string;
  c: Deno.UnsafePointer;
  d: number;
  f: BigInt64Array;
  g: ExampleInnerStruct;
}): Uint8Array {
  const __array_buffer = new ArrayBuffer(152);
  const __u8_view = new Uint8Array(__array_buffer);
  const __u64_view = new BigUint64Array(__array_buffer);
  const __i8_view = new Int8Array(__array_buffer);
  __u64_view[0] = Deno.UnsafePointer.of(new Uint16Array([__data.a])).value;
  __u64_view[1] =
    Deno.UnsafePointer.of(__cstring_encoder.encode(__data.b + "\0")).value;
  __u64_view[2] = __data.c.value;
  __i8_view[24] = __data.d;
  __u8_view.set(new Uint8Array(__data.f.buffer), 32);
  __u8_view.set(
    new Uint8Array(__into_ExampleInnerStruct(__data.g).buffer),
    136,
  );
  return __u8_view;
}
export function test(parameter0: bigint): {
  a: number;
  b: string;
  c: Deno.UnsafePointer;
  d: number;
  f: BigInt64Array;
  g: ExampleInnerStruct;
} {
  return __from_63131302a3ae1784(library.symbols.test(parameter0));
}
