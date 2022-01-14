import * as Plug from "https://deno.land/x/plug/mod.ts";
export const library = await Plug.prepare({ name: "test", url: "abcdef" }, { test: { parameters: ["usize", "pointer"], result: "pointer", nonblocking: false } });
const __cstring_encoder = new TextEncoder();
export interface ExampleStruct {
a: number;
b: number;
c: number;
}
function __example_struct_into(__example_struct: ExampleStruct): Deno.UnsafePointer {
const __array_buffer = new ArrayBuffer(12);
const __u8_view = new Uint8Array(__array_buffer);
const __u32_view = new Uint32Array(__array_buffer);
__u8_view[0] = __example_struct.a;
__u32_view[4] = __example_struct.b;
__u8_view[8] = __example_struct.c;
return Deno.UnsafePointer.of(new Uint8Array(__array_buffer));
}
export function test(parameter0: bigint, parameter1: string): ExampleStruct {
return __example_struct_from(library.symbols.test(parameter0, Deno.UnsafePointer.of(__cstring_encoder.encode(parameter1 + ""))));
}
