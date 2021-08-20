import { add } from "./bindings/bindings.ts";
import { assertEquals } from "https://deno.land/std/testing/asserts.ts";

Deno.test({
  name: "add#test",
  fn: () => {
    assertEquals(add(1, 2), 3);
    assertEquals(add(-1, 1), 0);
  },
});
