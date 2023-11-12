const _: () = {
    #[deno_bindgen::linkme::distributed_slice(deno_bindgen::INVENTORY)]
    pub static _A: deno_bindgen::Inventory = deno_bindgen::Inventory::Symbol(deno_bindgen::Symbol {
        name: stringify!(is_utf8),
        parameters: &[deno_bindgen::Type::Pointer, deno_bindgen::Type::Uint64],
        return_type: deno_bindgen::Type::Int32,
        non_blocking: false,
        internal: false,
        is_constructor: false,
    });
};
#[no_mangle]
extern "C" fn is_utf8(__arg_0: *const (), len: usize) -> i32 {
    fn is_utf8(ptr: *const u8, len: usize) -> i32 {
        std::str::from_utf8(unsafe { std::slice::from_raw_parts(ptr, len) }).is_ok()
            as i32
    }
    let ptr = __arg_0 as _;
    let ret = is_utf8(ptr, len);
    ret
}
