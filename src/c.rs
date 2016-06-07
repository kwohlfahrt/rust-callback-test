use libc::{c_int, c_void};

#[repr(C)]
pub struct Callback {
    user_data: c_void,
    callback: extern "C" fn(c_int, c_void) -> c_int,
    prev: *const Callback,
    next: *const Callback,
}

extern {
    pub fn add_callback(f: Option<extern fn (x: c_int, arg: *mut c_void) -> c_int>, arg: *mut c_void) -> *const Callback;
    pub fn invoke_callbacks(x: c_int) -> c_int;
    pub fn remove_callback(callback: *const Callback) -> *mut c_void;
}
