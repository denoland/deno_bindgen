fmt:
	cargo fmt
	deno fmt --ignore=target/,example/target/,example/bindings/

build:
	cargo build

test: build
	cd example && ../target/debug/deno_bindgen -o bindings/mod.ts && deno test -A --unstable

bench: build
	cd example && ../target/debug/deno_bindgen -o bindings/mod.ts && deno bench -A --unstable bench.js
