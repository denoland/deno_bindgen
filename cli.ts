// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

import { ensureDir } from "https://deno.land/std@0.132.0/fs/ensure_dir.ts";
import { parse } from "https://deno.land/std@0.132.0/flags/mod.ts";
import { join } from "https://deno.land/std@0.132.0/path/mod.ts";
import { codegen } from "./codegen.ts";

const flags = parse(Deno.args, { "--": true });
const release = !!flags.release;

const fetchPrefix = typeof flags.release == "string"
  ? flags.release
  : "../target/" + (release ? "release" : "debug");

const metafile = join(Deno.env.get("OUT_DIR") || "", "bindings.json");

async function build() {
  const cmd = ["cargo", "build"];
  if (release) cmd.push("--release");
  cmd.push(...flags["--"]);
  const proc = Deno.run({ cmd });
  return proc.status();
}

let source = null;
async function generate() {
  let conf;
  try {
    conf = JSON.parse(await Deno.readTextFile(metafile));
  } catch (_) {
    // Nothing to update.
    return;
  }

  const pkgName = conf.name;

  source = "// Auto-generated with deno_bindgen\n";
  source += codegen(
    fetchPrefix,
    pkgName,
    conf.typeDefs,
    conf.tsTypes,
    conf.symbols,
    {
      le: conf.littleEndian,
      release,
    },
  );

  await Deno.remove(metafile);
}

try {
  await Deno.remove(metafile);
} catch (e) {
  // no op
}

const status = await build().catch((_) => Deno.removeSync(metafile));
if (status?.success || typeof flags.release == "string") {
  await generate();
  if (source) {
    await ensureDir("bindings");
    await Deno.writeTextFile("bindings/bindings.ts", source);
  }
}

Deno.exit(status?.code || 0);
