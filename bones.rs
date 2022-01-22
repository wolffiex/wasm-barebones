use std::collections::HashMap;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;

static mut MAP: Option<HashMap<String, String>> = None;

extern {
    fn consoleLog(p: *mut c_char);
}

#[no_mangle]
pub fn alloc_bytes(size: i32) -> *mut c_char {
    let mut buf = vec![0 as u8; size as usize];
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

fn log(s: String) {
    let c_string = CString::new(s).unwrap();
    let p: *mut c_char = c_string.into_raw();
    unsafe {
        consoleLog(p);
    }
}

fn ensure_map() {
    unsafe {
        if MAP.is_none() {
            MAP = Some(HashMap::new());
            log("Initialized map".to_string());
        }
    }
}

#[no_mangle]
pub extern fn write(c_key: *mut c_char, c_value: *mut c_char) -> bool {
    ensure_map();
    let result: Option<String> = unsafe {
        let key = CString::from_raw(c_key).into_string().unwrap();
        let value = CString::from_raw(c_value).into_string().unwrap();
        log(format!("write {} {}", key, value));
        if let Some(map) = &mut MAP {
            map.insert(key, value)
        } else {
            panic!("Map uninitialized.");
        }
    };

    match result {
        Some(_) => true,
        None => false,
    }
}

#[no_mangle]
pub extern fn read(c_key: *mut c_char) -> *mut c_char {
    ensure_map();
    unsafe {
        let key = CString::from_raw(c_key).into_string().unwrap();
        if let Some(map) = &MAP {
            let result: &str = match map.get(&key) {
                Some(v) => v,
                None => "",
            };
            log(format!("get {}:{:?}", key, map.get(&key)));
            CString::new(result).unwrap().into_raw()
        } else {
            panic!("Map uninitialized.");
        }
    }
}
