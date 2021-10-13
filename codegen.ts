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

type TypeDef = Record<string, Record<string, string>>;

function resolveType(typeDefs: TypeDef, type: any): string {
  const t = typeof type == "string" ? type : type.struct.ident;
  if (Type[t] !== undefined) return Type[t];
  if (Object.keys(typeDefs).find((f) => f == t) !== undefined) {
    return t;
  }
  return "any";
}

function resolveDlopenParameter(typeDefs: TypeDef, type: any): string {
  const t = typeof type == "string" ? type : type.struct.ident;
  if (Type[t] !== undefined) return t;
  if (Object.keys(typeDefs).find((f) => f == t) !== undefined) {
    return "buffer";
  }
  throw new TypeError(`Type not supported: ${t}`);
}

type Sig = Record<string, {
  parameters: any[];
  result: string;
}>;

type Options = {
  le?: boolean;
};

export function codegen(
  target: string,
  decl: TypeDef,
  signature: Sig,
  options?: Options,
) {
  return `
const encode = (s: string) => new TextEncoder().encode(s);
const _lib = Deno.dlopen("${target}", { ${
    Object.keys(signature).map((sig) =>
      `${sig}: { parameters: [ ${
        signature[sig].parameters.map((p) => {
          const ffiParam = resolveDlopenParameter(decl, p);
          // FIXME: Dupe logic here.
          return `"${ffiParam}"${
            typeof p !== "string"
              ? `, "usize"`
              : ""
          }`;
        })
          .join(", ")
      } ], result: "${signature[sig].result}" }`
    ).join(", ")
  } });
${
    Object.keys(decl).map((def) =>
      `type ${def} = { ${
        Object.keys(decl[def]).map((f) =>
          `${f}: ${resolveType(decl, decl[def][f])}`
        ).join("; ")
      } };`
    ).join("\n")
  }
${
    Object.keys(signature).map((sig) =>
      `export function ${sig}(${
        signature[sig].parameters.map((p, i) =>
          `a${i}: ${resolveType(decl, p)}`
        ).join(",")
      }) {
  ${
        signature[sig].parameters.map((p, i) =>
          typeof p !== "string"
            ? `const a${i}_buf = encode(JSON.stringify(a${i}));`
            : null
        ).filter((c) => c !== null).join("\n")
      }
  return _lib.symbols.${sig}(${
        signature[sig].parameters.map((p, i) =>
          typeof p !== "string" ? `a${i}_buf, a${i}_buf.byteLength` : `a${i}`
        ).join(", ")
      }) as ${resolveType(decl, signature[sig].result)}
}`
    ).join("\n")
  }
 `;
}
