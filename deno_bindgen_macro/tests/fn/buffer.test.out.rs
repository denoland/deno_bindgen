const _: () = {
    #[deno_bindgen::linkme::distributed_slice(deno_bindgen::INVENTORY)]
    pub static _A: deno_bindgen::Inventory = deno_bindgen::Inventory::Symbol(deno_bindgen::Symbol {
        name: stringify!(write_hello),
        parameters: &[deno_bindgen::Type::Buffer],
        return_type: deno_bindgen::Type::Void,
        non_blocking: false,
        internal: false,
        is_constructor: false,
    });
};
#[no_mangle]
extern "C" fn write_hello(__arg_0: *const (), __arg_1: u32) {
    fn write_hello(buf: &mut [u8]) {
        buf[0] = b'H';
        buf[1] = b'e';
        buf[2] = b'l';
        buf[3] = b'l';
        buf[4] = b'o';
    }
    let buf = unsafe { std::slice::from_raw_parts_mut(__arg_0 as _, __arg_1 as usize) };
    let ret = write_hello(buf);
    ret
}
