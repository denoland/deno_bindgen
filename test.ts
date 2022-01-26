import { Plug } from "https://deno.land/x/plug/mod.ts";
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
  inner_a: number;
  inner_b: Deno.UnsafePointer;
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
      ? new Deno.UnsafePointerView(__source).getArrayBuffer(12)
      : __source instanceof Deno.UnsafePointerView
      ? __source.getArrayBuffer(12)
      : undefined)!;
  const __data_view = new DataView(__array_buffer);
  return {
    inner_a: __data_view.getUint32(0),
    inner_b: new Deno.UnsafePointer(__data_view.getBigUint64(4)),
  };
}
function __into_ExampleInnerStruct(__data: ExampleInnerStruct): Uint8Array {
  const __array_buffer = new ArrayBuffer(12);
  const __u8_array = new Uint8Array(__array_buffer);
  const __data_view = new DataView(__array_buffer);
  __data_view.setUint32(0, __data.inner_a);
  __data_view.setBigUint64(4, __data.inner_b.value);
  return __u8_array;
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
  const __data_view = new DataView(__array_buffer);
  return {
    a: new Deno.UnsafePointerView(
      new Deno.UnsafePointer(__data_view.getBigUint64(0)),
    ).getUint16(0),
    b: new Deno.UnsafePointerView(
      new Deno.UnsafePointer(__data_view.getBigUint64(8)),
    ).getCString(),
    c: new Deno.UnsafePointer(__data_view.getBigUint64(16)),
    d: __data_view.getInt8(24),
    f: new BigInt64Array(__array_buffer.slice(32, 136)),
    g: __from_ExampleInnerStruct(__array_buffer.slice(136, 152)),
  };
}
function __into_ExampleStruct(__data: ExampleStruct): Uint8Array {
  const __array_buffer = new ArrayBuffer(152);
  const __u8_array = new Uint8Array(__array_buffer);
  const __data_view = new DataView(__array_buffer);
  __data_view.setBigUint64(
    0,
    Deno.UnsafePointer.of(new Uint16Array([__data.a])).value,
  );
  __data_view.setBigUint64(
    8,
    Deno.UnsafePointer.of(__cstring_encoder.encode(__data.b + "\0")).value,
  );
  __data_view.setBigUint64(16, __data.c.value);
  __data_view.setInt8(24, __data.d);
  __u8_array.set(new Uint8Array(__data.f.buffer), 32);
  __u8_array.set(
    new Uint8Array(__into_ExampleInnerStruct(__data.g).buffer),
    136,
  );
  return __u8_array;
}
export type TestTuple = [number, string, Deno.UnsafePointer];
function __from_TestTuple(
  __source:
    | ArrayBuffer
    | Uint8Array
    | Deno.UnsafePointer
    | Deno.UnsafePointerView,
): TestTuple {
  const __array_buffer =
    (__source instanceof ArrayBuffer
      ? __source
      : __source instanceof Uint8Array
      ? __source.buffer
      : __source instanceof Deno.UnsafePointer
      ? new Deno.UnsafePointerView(__source).getArrayBuffer(24)
      : __source instanceof Deno.UnsafePointerView
      ? __source.getArrayBuffer(24)
      : undefined)!;
  const __data_view = new DataView(__array_buffer);
  return [
    new Deno.UnsafePointerView(
      new Deno.UnsafePointer(__data_view.getBigUint64(0)),
    ).getUint16(0),
    new Deno.UnsafePointerView(
      new Deno.UnsafePointer(__data_view.getBigUint64(8)),
    ).getCString(),
    new Deno.UnsafePointer(__data_view.getBigUint64(16)),
  ];
}
function __into_TestTuple(__data: TestTuple): Uint8Array {
  const __array_buffer = new ArrayBuffer(24);
  const __u8_array = new Uint8Array(__array_buffer);
  const __data_view = new DataView(__array_buffer);
  __data_view.setBigUint64(
    0,
    Deno.UnsafePointer.of(new Uint16Array([__data[0]])).value,
  );
  __data_view.setBigUint64(
    8,
    Deno.UnsafePointer.of(__cstring_encoder.encode(__data[1] + "\0")).value,
  );
  __data_view.setBigUint64(16, __data[2].value);
  return __u8_array;
}
export function test(parameter0: bigint): ExampleStruct {
  return __from_ExampleStruct(library.symbols.test(parameter0));
}
