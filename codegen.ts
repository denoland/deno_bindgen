const Type: Record<string, string> = {
  void: "null",
  i8: "number",
  u8: "number",
  i16: "number",
  u16: "number",
  i32: "number",
  u32: "number",
  i64: "number",
  u64: "number",
  usize: "number",
  isize: "number",
  f32: "number",
  f64: "number",
};

const BufferTypes: Record<string, string> = {
  str: "string",
  buffer: "Uint8Array",
};

enum Encoder {
  JsonStringify = "JSON.stringify",
  None = "",
}

const BufferTypeEncoders: Record<keyof typeof BufferTypes, Encoder> = {
  str: Encoder.None,
  buffer: Encoder.None,
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
  if (
    BufferTypes[t] !== undefined ||
    Object.keys(typeDefs).find((f) => f == t) !== undefined
  ) {
    return "buffer";
  }
  throw new TypeError(`Type not supported: ${t}`);
}

type Sig = Record<string, {
  parameters: any[];
  result: string;
  nonBlocking?: boolean;
}>;

type Options = {
  le?: boolean;
};

function isBufferType(p: any) {
  return typeof p !== "string" || BufferTypes[p] !== undefined;
}

// @littledivy is a dumb kid!
// he just can't make a interface
// for bindings.json
export function codegen(
  fetchPrefix: string,
  name: string,
  decl: TypeDef,
  typescript: Record<string, string>,
  signature: Sig,
  options?: Options,
) {
  return `import { Plug } from "https://deno.land/x/plug@0.4.0/mod.ts";
function encode(v: string | Uint8Array): Uint8Array {
  if (typeof v !== "string") return v;
  return new TextEncoder().encode(v);
}
const opts = {
  name: "${name}",
  url: "${fetchPrefix}"
};
const _lib = await Plug.prepare(opts, {
  ${
    Object.keys(signature).map((sig) =>
      `${sig}: { parameters: [ ${
        signature[sig].parameters.map((p) => {
          const ffiParam = resolveDlopenParameter(decl, p);
          // FIXME: Dupe logic here.
          return `"${ffiParam}"${isBufferType(p) ? `, "usize"` : ""}`;
        })
          .join(", ")
      } ], result: "${signature[sig].result}", nonblocking: ${
        String(!!signature[sig].nonBlocking)
      } }`
    ).join(", ")
  } });
${Object.keys(decl).map((def) => typescript[def]).join("\n")}
${
    Object.keys(signature).map((sig) =>
      `export function ${sig}(${
        signature[sig].parameters.map((p, i) =>
          `a${i}: ${resolveType(decl, p)}`
        ).join(",")
      }) {
  ${
        signature[sig].parameters.map((p, i) =>
          isBufferType(p)
            ? `const a${i}_buf = encode(${
              BufferTypeEncoders[p] ?? Encoder.JsonStringify
            }(a${i}));`
            : null
        ).filter((c) => c !== null).join("\n")
      }
  return _lib.symbols.${sig}(${
        signature[sig].parameters.map((p, i) =>
          isBufferType(p) ? `a${i}_buf, a${i}_buf.byteLength` : `a${i}`
        ).join(", ")
      }) as ${
        signature[sig].nonBlocking
          ? `Promise<${resolveType(decl, signature[sig].result)}>`
          : resolveType(decl, signature[sig].result)
      }
}`
    ).join("\n")
  }
 `;
}
