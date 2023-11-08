import {
  add,
  add2,
  bytelen,
  buf_mut,
  cstr,
  strlen,
  non_blocking,
  make_foo,
  inc_foo,
  Foo,  
} from "./bindings/bindings.ts";
import { assert, assertEquals } from "https://deno.land/std@0.178.0/testing/asserts.ts";

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

Deno.test({
  name: "make_foo#test",
  fn: () => {
    const foo = make_foo();
    assert(foo instanceof Foo);
    assertEquals(foo.bar(1), 43);
  },
})

Deno.test({
  name: "Foo#constructor",
  fn() {
    const foo = new Foo(42);
    assertEquals(foo.bar(1), 43);
  }
})

Deno.test({
  name: "Foo#using",
  fn() {
    using foo = new Foo(1);
    foo.inc();
    assertEquals(foo.bar(1), 3);
  }
});

Deno.test({
  name: "Foo#using explicit",
  fn() {
    using foo = make_foo();
    
    // Multiple dipose calls are nop.
    foo[Symbol.dispose]();
    foo[Symbol.dispose]();
  }
});

Deno.test({
  name: "inc_foo#test",
  fn: () => {
    using foo = new Foo(22);
    inc_foo(foo);
    assertEquals(foo.bar(0), 23);
  }
})