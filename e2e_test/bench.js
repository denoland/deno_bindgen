import { add, bytelen } from "./bindings/bindings.ts";

Deno.bench("add", () => add(1, 2));

const b = new Uint8Array([1, 2, 3, 4]);
Deno.bench("bytelen", () => bytelen(b));
