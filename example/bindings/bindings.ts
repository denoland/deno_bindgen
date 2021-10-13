// Auto-generated with deno_bindgen

const encode = (s: string) => new TextEncoder().encode(s);
const _lib = Deno.dlopen("target/debug/libadd.so", {
  add: { parameters: ["i32", "i32"], result: "i32" },
  add2: { parameters: ["buffer", "usize"], result: "i32" },
});
type Input = { b: number; a: number };
export function add(a0: number, a1: number) {
  return _lib.symbols.add(a0, a1) as number;
}
export function add2(a0: Input) {
  const a0_buf = encode(JSON.stringify(a0));

  return _lib.symbols.add2(a0_buf, a0_buf.byteLength) as number;
}
