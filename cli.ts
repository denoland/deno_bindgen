import { ensureDir } from "https://deno.land/std@0.105.0/fs/ensure_dir.ts";
import { parse } from "https://deno.land/std@0.105.0/flags/mod.ts";
import { codegen } from "./codegen.ts";

const flags = parse(Deno.args, { "--": true });
const release = !!flags.release;
const profile = release ? "release" : "debug";

async function build() {
  const cmd = ["cargo", "build"];
  if (release) cmd.push("--release");
  cmd.push(...flags["--"]);
  const proc = Deno.run({ cmd });
  await proc.status();
}

let ext = ".so";
if (Deno.build.os == "windows") {
  ext = ".dll";
} else if (Deno.build.os == "darwin") {
  ext = ".dylib";
}

let source = null;
async function generate() {
  let conf;
  try {
    conf = JSON.parse(await Deno.readTextFile("bindings.json"));
  } catch (_) {
    // Nothing to update.
    return;
  }

  console.log(conf);
  const pkgName = conf.name;

  source = "// Auto-generated with deno_bindgen\n";
  source += codegen(
    `target/${profile}/lib${pkgName}${ext}`,
    conf.type_defs,
    conf.bindings,
    {
      le: conf.le,
    }
  );
  await Deno.remove("bindings.json");
}

await build();
await generate();

if (source != null) {
 
  await ensureDir("bindings");
  await Deno.writeTextFile("bindings/bindings.ts", source);
}
