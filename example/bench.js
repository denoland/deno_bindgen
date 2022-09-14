import { add, test_buf, test_serde, test_str } from "./bindings/bindings.ts";

// Optimized fast paths:
Deno.bench("add", () => add(1, 2));
const b = new Uint8Array([1, 2, 3, 4]);
Deno.bench("test_buf", () => test_buf(b));

// Unoptimized paths:
Deno.bench("test_str", () => test_str("hello"));
Deno.bench("test_serde", () => {
  test_serde({ arr: ["IT", "WORKS"] });
});
