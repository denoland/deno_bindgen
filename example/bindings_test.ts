import {
  add,
  add2,
  bytelen,
  buf_mut,
  cstr,
  strlen,
  non_blocking,
} from "./bindings/bindings.ts";
import { assert, assertEquals } from "https://deno.land/std/testing/asserts.ts";

Deno.test({
  name: "add#test",
  fn: () => {
    assertEquals(add(1, 2), 3);
    assertEquals(add2(-1, 1), 0);
  },
});

Deno.test({
  name: "bytelen#test",
  fn: () => {
    assertEquals(bytelen(new TextEncoder().encode("hello")), 5);
  },
});

Deno.test({
  name: "buf_mut#test",
  fn: () => {
    const buf = new Uint8Array(1);
    buf_mut(buf);
    assertEquals(buf[0], 99);
  }
});

Deno.test({
  name: "cstr#test",
  fn: () => {
    const ptr = cstr();
    const str = Deno.UnsafePointerView.getCString(ptr!);
    assertEquals(str, "Hello, World!");
  },
});

Deno.test({
  name: "strlen#test",
  fn: () => {
    const ptr = strlen(cstr());
    assertEquals(ptr, 13);
  },
});


Deno.test({
  name: "non_blocking#test",
  fn: async () => {
    const result = await non_blocking();
    assertEquals(result, 42);
  },
});