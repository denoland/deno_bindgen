import {
  add,
  add2,
  test_mixed,
  test_mixed_order,
  test_serde,
} from "./bindings/bindings.ts";
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

Deno.test({
  name: "test_mixed#test",
  fn: () => {
    assertEquals(test_mixed(10, { a: 10, b: 20 }), 20);
  },
});

Deno.test({
  name: "test_serde#test",
  fn: () => {
    assertEquals(test_serde({ arr: ["IT", "WORKS"] }), 1);
  },
});

Deno.test({
  name: "test_mixed_order#test",
  fn: () => {
    assertEquals(test_mixed_order(10, { a: 10, b: 0 }, 10), 30);
  },
});
