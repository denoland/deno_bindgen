// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

import { ensureDir } from "https://deno.land/std@0.132.0/fs/ensure_dir.ts";
import { parse } from "https://deno.land/std@0.132.0/flags/mod.ts";
import { join } from "https://deno.land/std@0.132.0/path/mod.ts";
import { relative } from "https://deno.land/std@0.132.0/path/mod.ts";
import { codegen } from "./codegen.ts";

const flags = parse(Deno.args, { "--": true, alias: { f: "force" } });
const release = !!flags.release;
const force = !!flags.force;

const metafile = join(
  Deno.env.get("OUT_DIR") || await findRelativeTarget(),
  "bindings.json",
);

async function build() {
  if (force) {
    await Deno.run({
      cmd: ["cargo", "clean"],
    }).status();
  }
  const cmd = ["cargo", "build"];
  if (release) cmd.push("--release");
  cmd.push(...flags["--"]);
  const proc = Deno.run({ cmd });
  return proc.status();
}

async function findRelativeTarget() {
  const p = Deno.run({
    cmd: ["cargo", "metadata", "--format-version", "1"],
    stdout: "piped",
  });
  const metadata = JSON.parse(new TextDecoder().decode(await p.output()));
  return relative(Deno.cwd(), metadata.workspace_root);
}

let source = null;
async function generate() {
  let conf;
  try {
    conf = JSON.parse(await Deno.readTextFile(metafile));
  } catch (_) {
    // Nothing to update.
    return console.log(
      "No changes to rust code. Run with --force or -f to force an update.",
    );
  }

  const pkgName = conf.name;
  const fetchPrefix = typeof flags.release == "string" ? flags.release : join(
    await findRelativeTarget(),
    "../target",
    release ? "release" : "debug",
  );

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
