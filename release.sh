cd deno_bindgen_macro && cargo publish
# wait for crates.io to update
sleep 2
cargo publish
# update lockfile
make test
