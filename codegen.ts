// Copyright 2020-2023 the Deno authors. All rights reserved. MIT license.
// deno-lint-ignore-file no-explicit-any no-extra-boolean-cast

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

const tsFormatter = createFromBuffer(Deno.readFileSync(file.path));

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
  if (BufferTypes[t] !== undefined) {
    return "buffer";
  }
  if (
    Object.keys(typeDefs).find((f) => f == t) !== undefined
  ) {
    return "buffer";
  } else {
    return "pointer";
  }
  // deno-lint-ignore no-unreachable
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
  releaseURL: string | undefined;
};

function isTypeDef(p: any) {
  return typeof p !== "string";
}

function isBufferType(p: any) {
  return isTypeDef(p) || BufferTypes[p] !== undefined;
}

// deno-lint-ignore no-unused-vars
function needsPointer(p: any) {
  return isBufferType(p) && p !== "buffer" && p !== "buffermut";
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

  const functions = {
    encode: false,
    decode: false,
    readPointer: false,
  };

  return tsFormatter.formatText(
    "bindings.ts",
    `
const url = new URL("${fetchPrefix}", import.meta.url);
${
      typeof options?.releaseURL === "string"
        ? `
import { dlopen, FetchOptions } from "https://deno.land/x/plug@1.0.2/mod.ts";
let uri = url.toString();
if (!uri.endsWith("/")) uri += "/";

const darwin: string | { aarch64: string; x86_64: string } = uri;

const opts: FetchOptions = {
  name: "${name}",
  url: {
    darwin,
    windows: uri,
    linux: uri,
  },
  suffixes: {
    darwin: {
      aarch64: "_arm64",
    },
  },
  cache: ${!!options?.release ? '"use"' : '"reloadAll"'},
};
const { symbols } = await dlopen(opts, {
  `
        : `
let uri = url.pathname;
if (!uri.endsWith("/")) uri += "/";

// https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibrarya#parameters
if (Deno.build.os === "windows") {
  uri = uri.replace(/\\//g, "\\\\");
  // Remove leading slash
  if (uri.startsWith("\\\\")) {
    uri = uri.slice(1);
  }
}

const { symbols } = Deno.dlopen({
  darwin: uri + "lib${name}.dylib",
  windows: uri + "${name}.dll",
  linux: uri + "lib${name}.so",
  freebsd: uri + "lib${name}.so",
  netbsd: uri + "lib${name}.so",
  aix: uri + "lib${name}.so",
  solaris: uri + "lib${name}.so",
  illumos: uri + "lib${name}.so",
}[Deno.build.os], {`
    }
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
              .map((p, i) => {
                if (isBufferType(p)) {
                  functions.encode = true;
                  return `const a${i}_buf = encode(${
                    BufferTypeEncoders[p] ?? Encoder.JsonStringify
                  }(a${i}));`;
                }
                return null;
              })
              .filter((c) => c !== null)
              .join("\n")
          }

  const rawResult = symbols.${sig}(${
            parameters
              .map((p, i) => (isBufferType(p)
                ? `a${i}_buf, a${i}_buf.byteLength`
                : `a${i}`)
              )
              .join(", ")
          });
  ${
            (() => {
              if (isBufferType(result)) {
                functions.readPointer = true;
                return nonBlocking
                  ? `const result = rawResult.then(readPointer);`
                  : `const result = readPointer(rawResult);`;
              }
              return "const result = rawResult;";
            })()
          };
  ${
            (() => {
              if (isTypeDef(result)) {
                functions.decode = true;
                return nonBlocking
                  ? `return result.then(r => JSON.parse(decode(r))) as Promise<${
                    resolveType(
                      decl,
                      result,
                    )
                  }>;`
                  : `return JSON.parse(decode(result)) as ${
                    resolveType(decl, result)
                  };`;
              }
              if (result == "str") {
                functions.decode = true;
                return nonBlocking
                  ? "return result.then(decode);"
                  : "return decode(result);";
              }
              return "return result;";
            })()
          }`;
        })
        .join("\n")
    }
    ${
      functions.encode
        ? `
    function encode(v: string | Uint8Array): Uint8Array {
      if (typeof v !== "string") return v;
      return new TextEncoder().encode(v);
    }`
        : ""
    }
    
    ${
      functions.decode
        ? `
    function decode(v: Uint8Array): string {
      return new TextDecoder().decode(v);
    }`
        : ""
    }

    
    ${
      functions.readPointer
        ? `
    // deno-lint-ignore no-explicit-any
    function readPointer(v: any): Uint8Array {
      const ptr = new Deno.UnsafePointerView(v);
      const lengthBe = new Uint8Array(4);
      const view = new DataView(lengthBe.buffer);
      ptr.copyInto(lengthBe, 0);
      const buf = new Uint8Array(view.getUint32(0));
      ptr.copyInto(buf, 4);
      return buf;
    }
    `
        : ""
    }
    
 `,
  );
}
