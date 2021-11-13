import {
  add,
  add2,
  OptionStruct,
  sleep,
  test_buf,
  test_mixed,
  test_mixed_order,
  test_mut_buf,
  test_serde,
  test_str,
  test_lifetime,
} from "./bindings/bindings.ts";
import { assert, assertEquals } from "https://deno.land/std/testing/asserts.ts";

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

Deno.test({
  name: "test_options#test",
  fn: () => {
    let opts: OptionStruct = { maybe: " " };
    opts.maybe = null;
    opts.maybe = undefined;
  },
});

Deno.test({
  name: "test_str#test",
  fn: () => {
    let str = "Hello, World!";
    test_str(str);
  },
});

Deno.test({
  name: "test_buf#test",
  fn: () => {
    let buf = new Uint8Array([1, 0, 1]);
    assertEquals(test_buf(buf), 1);
  },
});

Deno.test({
  name: "sleep#test",
  fn: async () => {
    const ms = 100;
    const start = performance.now();
    const promise = sleep(ms).then(() => assert(start >= ms));
    assert(performance.now() - start < ms);
    await promise;
  },
});

Deno.test({
  name: "test_mut_buf#test",
  fn: () => {
    let u8 = new Uint8Array([0, 1, 2]);
    assertEquals(u8[0], 0);

    test_mut_buf(u8);
    assertEquals(u8[0], 69);
  },
});

Deno.test({
  name: "test_lifetime_struct#test",
  fn: () => {
    const TEXT = "Hello, World!";
    assertEquals(test_lifetime({ text: TEXT }), TEXT.length);
  }
})
