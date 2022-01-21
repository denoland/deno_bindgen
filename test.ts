import * as Plug from "https://deno.land/x/plug/mod.ts";
export const library = await Plug.prepare({ name: "test", url: "abcdef" }, {
  test: { parameters: ["usize"], result: "pointer", nonblocking: false },
});
const __cstring_encoder = new TextEncoder();
export interface ExampleStruct {
  a: number;
  b: string;
  c: Deno.UnsafePointer;
  d: number;
  f: BigInt64Array;
  g: ExampleInnerStruct;
}
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
function __from_ExampleInnerStruct(
  __source:
    | ArrayBuffer
    | Uint8Array
    | Deno.UnsafePointer
    | Deno.UnsafePointerView,
): ExampleInnerStruct {
  const __array_buffer =
    (__source instanceof ArrayBuffer
      ? __source
      : __source instanceof Uint8Array
      ? __source.buffer
      : __source instanceof Deno.UnsafePointer
      ? new Deno.UnsafePointerView(__source).getArrayBuffer(16)
      : __source instanceof Deno.UnsafePointerView
      ? __source.getArrayBuffer(16)
      : undefined)!;
  const __u64_view = new BigUint64Array(__array_buffer);
  const __i64_view = new BigInt64Array(__array_buffer);
  const __inner_a = new Deno.UnsafePointer(__u64_view[0]);
  const __inner_b = __i64_view[1];
  return {
    inner_a: __inner_a,
    inner_b: __inner_b,
  };
}
function __into_ExampleStruct(__data: ExampleStruct): Uint8Array {
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
function __from_ExampleStruct(
  __source:
    | ArrayBuffer
    | Uint8Array
    | Deno.UnsafePointer
    | Deno.UnsafePointerView,
): ExampleStruct {
  const __array_buffer =
    (__source instanceof ArrayBuffer
      ? __source
      : __source instanceof Uint8Array
      ? __source.buffer
      : __source instanceof Deno.UnsafePointer
      ? new Deno.UnsafePointerView(__source).getArrayBuffer(152)
      : __source instanceof Deno.UnsafePointerView
      ? __source.getArrayBuffer(152)
      : undefined)!;
  const __u64_view = new BigUint64Array(__array_buffer);
  const __i8_view = new Int8Array(__array_buffer);
  const __a = new Deno.UnsafePointerView(new Deno.UnsafePointer(__u64_view[0]))
    .getUint16(0);
  const __b = new Deno.UnsafePointerView(new Deno.UnsafePointer(__u64_view[1]))
    .getCString();
  const __c = new Deno.UnsafePointer(__u64_view[2]);
  const __d = __i8_view[24];
  const __f = new BigInt64Array(__array_buffer.slice(32, 136));
  const __g = __from_ExampleInnerStruct(__array_buffer.slice(136, 152));
  return {
    a: __a,
    b: __b,
    c: __c,
    d: __d,
    f: __f,
    g: __g,
  };
}
export function test(parameter0: bigint): ExampleStruct {
  return __from_ExampleStruct(library.symbols.test(parameter0));
}
