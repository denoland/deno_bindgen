import { print } from "https://deno.land/x/swc@0.1.4/mod.ts";

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

type TypeDef = {
  fields: Record<string, string>;
  ident: string;
};

function resolveType(typeDefs: TypeDef[], type: string): string {
  if (Type[type] !== undefined) return Type[type];
  if (typeDefs.find((f) => f.ident == type) !== undefined) {
    return type;
  }
  throw new TypeError(`Type not supported: ${type}`);
}

function resolveDlopenParameter(typeDefs: TypeDef[], type: string): string {
  if (Type[type] !== undefined) return type;
  if (typeDefs.find((f) => f.ident == type) !== undefined) {
    return "buffer";
  }
  throw new TypeError(`Type not supported: ${type}`);
}

type Sig = {
  func: string;
  parameters: { ident: string; type: string }[];
  result: string;
};

export function codegen(target: string, decl: TypeDef[], signature: Sig[]) {
  return `import { Struct, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64 } from "https://deno.land/x/byte_type/mod.ts";
const usize = u64;
const isize = i64;

const _lib = Deno.dlopen("${target}", { ${
    signature.map((sig) =>
      `${sig.func}: { parameters: [ ${
        sig.parameters.map((p) => `"${resolveDlopenParameter(decl, p.type)}"`)
          .join(", ")
      } ], result: "${sig.result}" }`
    ).join(", ")
  } });
${
    decl.map((def) =>
      `type ${def.ident} = { ${
        Object.keys(def.fields).map((f) =>
          `${f}: ${resolveType(decl, def.fields[f])}`
        ).join("; ")
      } };`
    ).join("\n")
  }
${
    decl.map((def) =>
      `const _${def.ident} = new Struct({ ${
        Object.keys(def.fields).map((f) => `${f}: ${def.fields[f]}`).join(", ")
      } });`
    ).join("\n")
  }
${
    signature.map((sig) =>
      `export function ${sig.func}(${
        sig.parameters.map((p) => `${p.ident}: ${resolveType(decl, p.type)}`)
          .join(", ")
      }) {
  ${
        sig.parameters.filter((p) => Type[p.type] == undefined).map((p) =>
          `const _buf_${p.ident} = new Uint8Array(_${p.type}.size);
  const _view_${p.ident} = new DataView(_buf_${p.ident}.buffer);
  _${p.type}.write(_view_${p.ident}, 0, ${p.ident});`
        ).join("\n")
      }
  const _result = _lib.symbols.${sig.func}(${
        sig.parameters.map((p) =>
          Type[p.type] == undefined ? ` _buf_${p.ident}` : p.ident
        ).join(", ")
      });
  return _result as ${resolveType(decl, sig.result)};
}`
    ).join("\n")
  }
  `;
}