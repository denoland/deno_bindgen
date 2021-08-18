// Auto-generated with deno_bindgen
const _lib = Deno.dlopen('target/debug/libadd.so', { add: { result: "i32", parameters: ["i32","i32"] } });
export function add(a0: number, a1: number): number { return _lib.symbols.add(a0, a1) as number; }
