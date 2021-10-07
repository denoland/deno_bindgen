import { add, add2 } from "./bindings/bindings.ts";
import { assertEquals } from "https://deno.land/std/testing/asserts.ts";

Deno.test({
  name: "add#test",
  fn: () => {
    assertEquals(add(1, 2), 3);
    assertEquals(add(-1, 1), 0);
  },
});

Deno.test({
  name: "add2#test",
  fn: () => {
    assertEquals(add2({ a: 1, b: 2 }), 3);
    
  },
});
