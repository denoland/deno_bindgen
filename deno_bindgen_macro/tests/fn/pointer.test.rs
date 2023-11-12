fn is_utf8(ptr: *const u8, len: usize) -> i32 {
    std::str::from_utf8(unsafe { std::slice::from_raw_parts(ptr, len) }).is_ok() as i32
}