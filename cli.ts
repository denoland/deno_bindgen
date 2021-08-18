import { ensureDir } from "https://deno.land/std@0.105.0/fs/ensure_dir.ts";
import { parse } from "https://deno.land/std@0.105.0/flags/mod.ts";

const flags = parse(Deno.args, { "--": true });
const release = !!flags.release;
const profile = release ? "release": "debug";

async function build() {
  const cmd = ["cargo", "build"];
  if(release) cmd.push("--release");
  cmd.push(...flags["--"]);
  const proc = Deno.run({ cmd, });
  await proc.status();
}

let ext = ".so";
if (Deno.build.os == "windows") {
  ext = ".dll";
} else if (Deno.build.os == "darwin") {
  ext = ".dylib";
}

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

function invalidType(type: string) {
  throw new TypeError(`Type not supported: ${type}`);
}

let source = null;
async function generate() {
  try {
    const conf = JSON.parse(await Deno.readTextFile("bindings.json"));
    const pkgName = conf.name;
    source = "// Auto-generated with deno_bindgen\n";
    source += `const _lib = Deno.dlopen('target/${profile}/lib${pkgName}${ext}', { ${
      conf.bindings.map((e: any) =>
        `${e.func}: { result: "${e.result}", parameters: [${
          e.parameters.map((p: any) => `"${p.type}"`)
        }] }`
      ).join(", ")
    } });\n`;
    for (let bindings of conf.bindings) {
      source += `export function ${bindings.func}(${
        bindings.parameters.map((p: any) =>
          `${p.ident}: ${Type[p.type] || invalidType(p.type)}`
        ).join(", ")
      }): ${Type[bindings.result]} { return _lib.symbols.${bindings.func}(${
        bindings.parameters.map((p: any) => p.ident).join(", ")
      }); }\n`;
    }
  } catch (_) {
    // It was a rerun
    return;
  }
  await Deno.remove("bindings.json");
}

await build();
await generate();

if (source != null) {
  await ensureDir("bindings");
  await Deno.writeTextFile("bindings/bindings.ts", source);
}
