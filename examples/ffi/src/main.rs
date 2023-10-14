use libc::c_char;
use std::ffi::CStr;
use std::str;

extern "C" {
    fn ccall() -> *const c_char;
}

#[no_mangle]
pub extern "C" fn rustcall() -> *const c_char {
    let c_str = std::ffi::CString::new("[Rust] Hello sir").unwrap();
    let ptr = c_str.as_ptr();
    std::mem::forget(c_str);
    ptr
}

fn main() {
    println!("[Rust] Receiving message...");
    let c_buf: *const c_char = unsafe { ccall() };
    println!("[Rust] Got It!");
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned(); // if necessary
    println!("[Rust] Message is: {}", str_buf);
    println!("[Rust] Sending Message to Python");
}
