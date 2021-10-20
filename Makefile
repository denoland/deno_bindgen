fmt:
	cargo fmt
	deno fmt --ignore=target/,example/target/,example/bindings/

test:
	cd example && deno_bindgen && deno test -A --unstable
