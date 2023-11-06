import { parse } from "https://deno.land/std@0.132.0/flags/mod.ts";

const flags = parse(Deno.args, { "--": true });
const release = !!flags.release;

function build() {
  const args = ["build"];
  if (release) args.push("--release");
  args.push(...flags["--"]);
  const proc = new Deno.Command("cargo", { args, stderr: "inherit" });
  return proc.outputSync();
}

function nm() {
  // Run `nm` to get the symbols from the compiled library.
  const args = [
    "nm",
    "--format=bsd",
    "--defined-only",
    "target/debug/libdeno_bindgen_test.dylib",
  ];
  if (release) args.push("--demangle");
  const proc = new Deno.Command("nm", { args, stdout: "piped" });
  const output = proc.outputSync();
  const stdout = new TextDecoder().decode(output.stdout);
  const symbols = stdout.split("\n").filter((s) => s.length > 0).slice(1);

  const symbols2 = symbols.map((s) => {
    const [addr, ty, name] = s.split(" ");
    return { addr, ty, name };
  }).filter((s) => s.name.startsWith("___de90_"));
  return symbols2;
}

function run_init(symbols) {
    const symbols_obj = {};
    symbols.forEach(({ name }) => {
        symbols_obj[name.slice(1)] = {
            parameters: ["buffer", "buffer"],
            result: "pointer",
        };
    });

    const lib = Deno.dlopen("./target/debug/libdeno_bindgen_test.dylib", symbols_obj);
    const params = new Uint8Array(20);
    const result = new Uint8Array(1);
    const processed = [];
    for (const fn in lib.symbols) {
        const name_ptr = lib.symbols[fn](params, result);
        const name = Deno.UnsafePointerView.getCString(name_ptr);
        processed.push({ name, params: [...params].map(p => C_TYPE[p]), result: C_TYPE[result[0]] }) 
    }
    return processed;
}

const C_TYPE = ["A", "B"];

function codegen(symbols) {
    let code = '';
    for (let i = 0; i < symbols.length; i++) {
        const { name, params, result } = symbols[i];
        const params_str = params.map((p, j) => `p${j}: ${p}`).join(', ');
        const params_idents = params.map((p, j) => `p${j}`).join(', ');
        code += `export function ${name}(${params_str}): ${result} { return lib.${name}(${params_idents}); }\n`;
    }

    console.log(code)
}

build();
const symbols = nm();
const processed = run_init(symbols);
codegen(processed);

