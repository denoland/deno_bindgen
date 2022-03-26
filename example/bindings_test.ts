import {
  add,
  add2,
  OptionStruct,
  sleep,
  test_buf,
  test_buffer_return,
  test_buffer_return_async,
  test_hashmap,
  test_lifetime,
  test_manual_ptr,
  test_manual_ptr_async,
  test_mixed,
  test_mixed_order,
  test_mut_buf,
  test_output,
  test_output_async,
  test_reserved_field,
  test_serde,
  test_str,
  test_str_ret,
  test_tag_and_content,
  TestReservedField,
  WithRecord,
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
  },
});

Deno.test({
  name: "test_tag_and_content#test",
  fn: () => {
    assertEquals(test_tag_and_content({ key: "A", value: { b: 10 } }), 10);

    // test_tag_and_content returns -1 when enum variant isn't TagAndContent::A
    assertEquals(test_tag_and_content({ key: "C", value: { d: 10 } }), -1);
  },
});

Deno.test({
  name: "test_buffer_return#test",
  fn: () => {
    const buf = test_buffer_return(
      new Uint8Array([1, 2, 3]),
    );

    assertEquals(buf.byteLength, 3);
    assertEquals(buf[0], 1);
    assertEquals(buf[1], 2);
    assertEquals(buf[2], 3);
  },
});

Deno.test({
  name: "test_buffer_return_async#test",
  fn: async () => {
    const buf = await test_buffer_return_async(
      new Uint8Array([1, 2, 3]),
    );

    assertEquals(buf.byteLength, 3);
    assertEquals(buf[0], 1);
    assertEquals(buf[1], 2);
    assertEquals(buf[2], 3);
  },
});

Deno.test({
  name: "test_manual_ptr#test",
  fn: () => {
    const buf = test_manual_ptr();
    const val = new TextDecoder().decode(buf);

    assertEquals(val, "test");
  },
});

Deno.test({
  name: "test_manual_ptr_async#test",
  fn: async () => {
    const buf = await test_manual_ptr_async();
    const val = new TextDecoder().decode(buf);

    assertEquals(val, "test");
  },
});

Deno.test({
  name: "test_output#test",
  fn: () => {
    const obj = test_output();

    assertEquals(obj.a, 1);
    assertEquals(obj.b, 2);
  },
});

Deno.test({
  name: "test_output_async#test",
  fn: async () => {
    const obj = await test_output_async();

    assertEquals(obj.a, 3);
    assertEquals(obj.b, 4);
  },
});

Deno.test({
  name: "test_reserved_field#test",
  fn: () => {
    const obj = test_reserved_field();

    assertEquals(obj.type, 1);
    assertEquals(obj.ref, 2);
  },
});

Deno.test({
  name: "test_str_ret#test",
  fn: () => {
    assertEquals(test_str_ret(), "🦕");
  },
});

Deno.test({
  name: "test_hashmap#test",
  fn: () => {
    assertEquals(test_hashmap(), { my_map: { "key": "value" } } as WithRecord);
  },
});
