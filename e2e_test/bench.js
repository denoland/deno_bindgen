import { add, bytelen, Foo, make_foo } from "./bindings/mod.ts";

Deno.bench("add", () => add(1, 2));

const b = new Uint8Array([1, 2, 3, 4]);
Deno.bench("bytelen", () => bytelen(b));

Deno.bench("make_foo", () => make_foo(21));
Deno.bench("new Foo", () => new Foo(21));

const foo = new Foo(21);
Deno.bench("Foo#bar", () => foo.bar(1));
