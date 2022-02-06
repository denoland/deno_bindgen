// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

import { ensureDir } from "https://deno.land/std@0.123.0/fs/ensure_dir.ts";
import { parse } from "https://deno.land/std@0.123.0/flags/mod.ts";
import { codegen } from "./codegen.ts";

const flags = parse(Deno.args, { "--": true });
const release = !!flags.release;

const fetchPrefix = typeof flags.release == "string"
  ? flags.release
  : "../target/" + (release ? "release" : "debug");

async function build() {
  const cmd = ["cargo", "build"];
  if (release) cmd.push("--release");
  cmd.push(...flags["--"]);
  const proc = Deno.run({ cmd });
  await proc.status();
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

  await Deno.remove("bindings.json");
}

try {
  await Deno.remove("bindings.json");
} catch (e) {
  // no op
}

await build().catch((_) => Deno.removeSync("bindings.json"));
await generate();

if (source != null) {
  await ensureDir("bindings");
  await Deno.writeTextFile("bindings/bindings.ts", source);
}
