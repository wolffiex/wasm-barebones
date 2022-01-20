use std::ffi::CString;
use std::mem;
// use std::ptr;
use std::os::raw::c_char;

extern "C" {
    fn consoleLog(p: *mut c_char);
}

#[no_mangle]
pub fn alloc_cstring(size: i32) -> *mut c_char {
    let mut buf = vec![0 as u8; size as usize + 1];
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

#[no_mangle]
pub extern fn add(x: i32, y: i32) -> i32 {
    log(format!("add {} {}", x, y));
    x + y
}

#[no_mangle]
pub extern fn write(c_key: *mut c_char, val: i32) {
    let key = unsafe {
        CString::from_raw(c_key).into_string()
    };

    log(format!("write {} {}", key.unwrap(), val));
}

fn log(s:String) {
    let c_string = CString::new(s).unwrap();
    let p: *mut c_char = c_string.into_raw();
    unsafe {
        consoleLog(p);
    }
}