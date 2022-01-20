use std::ffi::CString;
use std::mem;
// use std::ptr;
use std::os::raw::c_char;

#[no_mangle]
pub extern fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[no_mangle]
pub fn alloc_cstring(size: usize) -> *mut c_char {
    let mut buf = vec![0 as u8; size];
    let p = buf.as_mut_ptr();
    mem::forget(buf);
    p as *mut c_char
}

#[no_mangle]
fn dealloc_cstring(p: *mut c_char) {
    let _ = unsafe {
        CString::from_raw(p)
    };
}
