extern crate libc;
use libc::{c_void, c_char, c_int, RTLD_LAZY};
use std::ffi::CString;

extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *const c_void;
    fn dlsym(handle: *const c_void, symbol: *const c_char) -> *const c_void;
    fn dlclose(handle: *const c_void) -> c_int;
}

fn main() {
    let filename = CString::new("./cfile/libsample.so").unwrap();
    let funcname = CString::new("foo").unwrap();
    unsafe {
        let handle = dlopen(filename.as_ptr(), RTLD_LAZY);
        println!("handle: 0x{:>016x}", handle as u64);
        let func = dlsym(handle, funcname.as_ptr());
        let func = std::mem::transmute::<*const c_void, extern "C" fn() -> c_int>(func);
        println!("func: 0x{:>016x}", func as u64);
        let ret = func();
        println!("ret: {}", ret);
        dlclose(handle);
    }
}
