fmt:
	cargo fmt
	deno fmt --ignore=target/,example/target/,example/bindings/

test:
	cd example && deno run -A ../cli.ts && deno test -A --unstable

bench:
	cd example && deno run -A ../cli.ts && deno bench -A --unstable bench.js
