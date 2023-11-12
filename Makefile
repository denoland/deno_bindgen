fmt:
	cargo fmt
	deno fmt --ignore=target/,e2e_test/target/,e2e_test/bindings/

build:
	cargo build

test: build
	cargo test
	cd e2e_test && ../target/debug/deno_bindgen -o bindings/mod.ts && deno test -A --unstable

bench: build
	cd e2e_test && ../target/debug/deno_bindgen -o bindings/mod.ts && deno bench -A --unstable bench.js
