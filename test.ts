import { Plug } from "https://deno.land/x/plug/mod.ts";
const library = await Plug.prepare({ name: "test_lib", urls: { darwin: "https://example.com/some/path/libtest_lib.dylib", linux: "https://example.com/some/path/libtest_lib.so", windows: "https://example.com/some/path/test_lib.dll" } }, {  });
export interface TestStruct {
property: number;
pointer: number;
}function __from_TestStruct(__source: ArrayBuffer | Uint8Array | Deno.UnsafePointer | Deno.UnsafePointerView): TestStruct {
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
const __data_view = new DataView(__array_buffer);
return {
  property: __data_view.getFloat64(0),
  pointer: new Deno.UnsafePointerView(new Deno.UnsafePointer(__data_view.getBigUint64(8))).getFloat64(0)
};
}function __into_TestStruct(__data: TestStruct): Uint8Array {
const __array_buffer = new ArrayBuffer(16);
const __u8_array = new Uint8Array(__array_buffer);
        const __data_view = new DataView(__array_buffer);
__data_view.setFloat64(0, __data.property);
__data_view.setBigUint64(8, Deno.UnsafePointer.of(new Float64Array([__data.pointer])).value);
return __u8_array;
}no.UnsafePointer.of(new Float64Array([__data.pointer])).value);
return __u8_array;
}_data.pointer])).value,
  );
  return __u8_array;
}
