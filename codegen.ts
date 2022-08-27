// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

import {
  createFromBuffer,
  GlobalConfiguration,
} from "https://deno.land/x/dprint@0.2.0/mod.ts";
import * as Cache from "https://deno.land/x/cache@0.2.13/mod.ts";

Cache.configure({ directory: Cache.options.directory });
const cache = Cache.namespace("deno_bindgen_cli");

const globalConfig: GlobalConfiguration = {
  indentWidth: 2,
  lineWidth: 80,
};

const file = await cache.cache(
  "https://plugins.dprint.dev/typescript-0.57.0.wasm",
);

const tsFormatter = createFromBuffer(await Deno.readFile(file.path));

tsFormatter.setConfig(globalConfig, {
  semiColons: "asi",
});

const Type: Record<string, string> = {
  void: "null",
  i8: "number",
  u8: "number",
  i16: "number",
  u16: "number",
  i32: "number",
  u32: "number",
  i64: "bigint",
  u64: "bigint",
  usize: "bigint",
  isize: "bigint",
  f32: "number",
  f64: "number",
};

const BufferTypes: Record<string, string> = {
  str: "string",
  buffer: "Uint8Array",
  buffermut: "Uint8Array",
  ptr: "Uint8Array",
};

enum Encoder {
  JsonStringify = "JSON.stringify",
  None = "",
}

const BufferTypeEncoders: Record<keyof typeof BufferTypes, Encoder> = {
  str: Encoder.None,
  buffer: Encoder.None,
  buffermut: Encoder.None,
  ptr: Encoder.None,
};

type TypeDef = Record<string, Record<string, string>>;

function resolveType(typeDefs: TypeDef, type: any): string {
  const t = typeof type == "string" ? type : type.structenum.ident;
  if (Type[t] !== undefined) return Type[t];
  if (BufferTypes[t] !== undefined) return BufferTypes[t];
  if (Object.keys(typeDefs).find((f) => f == t) !== undefined) {
    return t;
  }
  return "any";
}

function resolveDlopenParameter(typeDefs: TypeDef, type: any): string {
  const t = typeof type == "string" ? type : type.structenum.ident;
  if (Type[t] !== undefined) return t;
  if (t === "buffer" || t === "buffermut") {
    return "buffer";
  }
  if (
    BufferTypes[t] !== undefined ||
    Object.keys(typeDefs).find((f) => f == t) !== undefined
  ) {
    return "pointer";
  }
  throw new TypeError(`Type not supported: ${t}`);
}

type Sig = Record<
  string,
  {
    parameters: any[];
    result: string;
    nonBlocking?: boolean;
  }
>;

type Options = {
  le?: boolean;
  release?: boolean;
};

function isTypeDef(p: any) {
  return typeof p !== "string";
}

function isBufferType(p: any) {
  return isTypeDef(p) || BufferTypes[p] !== undefined;
}

// TODO(@littledivy): factor out options in an interface
export function codegen(
  fetchPrefix: string,
  name: string,
  decl: TypeDef,
  typescript: Record<string, string>,
  signature: Sig,
  options?: Options,
) {
  signature = Object.keys(signature)
    .sort()
    .reduce(
      (acc, key) => ({
        ...acc,
        [key]: signature[key],
      }),
      {},
    );

  return tsFormatter.formatText(
    "bindings.ts",
    `import { CachePolicy, prepare } from "https://deno.land/x/plug@0.5.2/plug.ts";

function encode(v: string | Uint8Array): Uint8Array {
  if (typeof v !== "string") return v;
  return new TextEncoder().encode(v);
}

function decode(v: Uint8Array): string {
  return new TextDecoder().decode(v);
}

function readPointer(v: any): Uint8Array {
  const ptr = new Deno.UnsafePointerView(v as bigint)
  const lengthBe = new Uint8Array(4);
  const view = new DataView(lengthBe.buffer);
  ptr.copyInto(lengthBe, 0);
  const buf = new Uint8Array(view.getUint32(0));
  ptr.copyInto(buf, 4)
  return buf
}

const url = new URL("${fetchPrefix}", import.meta.url);
let uri = url.toString();
if (!uri.endsWith("/")) uri += "/";

let darwin: string | { aarch64: string; x86_64: string } = uri + "lib${name}.dylib";

if (url.protocol !== "file:") {
  // Assume that remote assets follow naming scheme
  // for each macOS artifact.
  darwin = {
    aarch64: uri + "lib${name}_arm64.dylib",
    x86_64: uri + "lib${name}.dylib",
  }
}

const opts = {
  name: "${name}",
  urls: {
    darwin,
    windows: uri + "${name}.dll",
    linux: uri + "lib${name}.so",
  },
  policy: ${!!options?.release ? "undefined" : "CachePolicy.NONE"},
};
const _lib = await prepare(opts, {
  ${
      Object.keys(signature)
        .map(
          (sig) =>
            `${sig}: { parameters: [ ${
              signature[sig].parameters
                .map((p) => {
                  const ffiParam = resolveDlopenParameter(decl, p);
                  // FIXME: Dupe logic here.
                  return `"${ffiParam}"${isBufferType(p) ? `, "usize"` : ""}`;
                })
                .join(", ")
            } ], result: "${
              resolveDlopenParameter(
                decl,
                signature[sig].result,
              )
            }", nonblocking: ${String(!!signature[sig].nonBlocking)} }`,
        )
        .join(", ")
    } });
${
      Object.keys(decl)
        .sort()
        .map((def) => typescript[def])
        .join("\n")
    }
${
      Object.keys(signature)
        .map((sig) => {
          const { parameters, result, nonBlocking } = signature[sig];

          return `export function ${sig}(${
            parameters
              .map((p, i) => `a${i}: ${resolveType(decl, p)}`)
              .join(",")
          }) {
  ${
            parameters
              .map((p, i) =>
                isBufferType(p)
                  ? `const a${i}_buf = encode(${
                    BufferTypeEncoders[p] ?? Encoder.JsonStringify
                  }(a${i}));`
                  : null
              )
              .filter((c) => c !== null)
              .join("\n")
          }
  let rawResult = _lib.symbols.${sig}(${
            parameters
              .map((p, i) => (isBufferType(p)
                ? `a${i}_buf, a${i}_buf.byteLength`
                : `a${i}`)
              )
              .join(", ")
          });
  ${
            isBufferType(result)
              ? nonBlocking
                ? `const result = rawResult.then(readPointer);`
                : `const result = readPointer(rawResult);`
              : "const result = rawResult;"
          };
  ${
            isTypeDef(result)
              ? nonBlocking
                ? `return result.then(r => JSON.parse(decode(r))) as Promise<${
                  resolveType(
                    decl,
                    result,
                  )
                }>;`
                : `return JSON.parse(decode(result)) as ${
                  resolveType(decl, result)
                };`
              : result == "str"
              ? nonBlocking
                ? "return result.then(decode);"
                : "return decode(result);"
              : "return result;"
          };
}`;
        })
        .join("\n")
    }
 `,
  );
}
