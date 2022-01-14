import * as Plug from "https://deno.land/x/plug/mod.ts";
export const library = await Plug.prepare({ name: "test", url: "abcdef" }, { test: { parameters: ["usize", "pointer"], result: "pointer", nonblocking: false } });
export interface ExampleStruct {
  test: number;
}
function __example_struct_into(__example_struct: ExampleStruct): Deno.UnsafePointer {
  const __array_buffer = new ArrayBuffer(8);
  const __u64_view = new BigUint64Array(__array_buffer); __u64_view[0] = __example_struct.test;
  return new Deno.UnsafePointer(new Uint8Array(__array_buffer));
}

const __cstring_encoder = new TextEncoder();
function __cstring_into(__cstring: string): Deno.UnsafePointer {
  const __buffer = new Uint8Array(__cstring.length + 1);
  __cstring_encoder.encodeInto(__cstring, __buffer);
  return Deno.UnsafePointer.of(__buffer);
}
export function test(parameter0: number, parameter1: string): ExampleStruct {
  return __example_struct_from(library.symbols.test(parameter0, __cstring_into(parameter1)));
}
