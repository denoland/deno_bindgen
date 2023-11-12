const _: () = {
    #[deno_bindgen::linkme::distributed_slice(deno_bindgen::INVENTORY)]
    pub static _A: deno_bindgen::Inventory = deno_bindgen::Inventory::Symbol(deno_bindgen::Symbol {
        name: stringify!(add),
        parameters: &[deno_bindgen::Type::Int32, deno_bindgen::Type::Int32],
        return_type: deno_bindgen::Type::Int32,
        non_blocking: false,
        internal: false,
        is_constructor: false,
    });
};
#[no_mangle]
extern "C" fn add(a: i32, b: i32) -> i32 {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let ret = add(a, b);
    ret
}
