//! imports/exports for WebAssembly functions
#![cfg(target_arch = "wasm32")]

#[link(wasm_import_module = "fl_imps")]
extern "C" {
    fn __console_log(ptr: *const u8, len: usize);
    fn __get_input_data(ptr: *const u8);
    fn __insert_error(ptr: *const u8, len: usize);
    fn __insert_response(ptr: *const u8, len: usize);
}

#[inline(never)]
pub fn console_log(s: &str) {
    unsafe {
        __console_log(s.as_ptr(), s.len());
    }
}

#[inline(never)]
pub fn get_input_data(req_len: i32) -> Vec<u8> {
    use std::slice;
    let buf: Vec<u8> = Vec::with_capacity(req_len as _);
    let req_ptr = buf.as_ptr();

    // https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html
    let slice = unsafe {
        __get_input_data(req_ptr);
        slice::from_raw_parts(req_ptr, req_len as _)
    };
    slice.to_vec()
}

#[inline(never)]
pub fn insert_response(r: &str) {
    unsafe {
        __insert_response(r.as_ptr(), r.len());
    }
}

#[inline(never)]
pub fn insert_error(e: &str) {
    unsafe {
        __insert_error(e.as_ptr(), e.len());
    }
}
