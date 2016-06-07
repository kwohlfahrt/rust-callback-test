extern crate libc;

use std::mem;

use libc::{c_int, c_void};

extern {
    fn set_handler(f: Option<extern fn (x: c_int, arg: *mut c_void) -> c_int>, arg: *mut c_void);
    fn invoke_handler(x: c_int) -> c_int;
    fn unset_handler() -> *mut c_void;
}

extern fn callback_handler(x: c_int, arg: *mut c_void) -> c_int {
    let closure: &mut Box<FnMut(i32) -> bool> = unsafe { mem::transmute(arg) };
    closure(x as i32) as c_int
}

pub fn set_callback<F>(callback: F) where F: FnMut(i32) -> bool, F: 'static {
    let cb: Box<Box<FnMut(i32) -> bool>> = Box::new(Box::new(callback));
    let ptr = Box::into_raw(cb) as * mut _;
    unsafe {
        set_handler(Some(callback_handler), ptr);
    }
}

pub fn invoke_callback(x: i32) -> bool {
    unsafe {
        invoke_handler(x as c_int) > 0
    }
}

pub fn unset_callback() {
    let ptr = unsafe { unset_handler() };
    // drop the callback
    let _: Box<Box<FnMut(i32) -> bool>> = unsafe { Box::from_raw(ptr as *mut _) };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_callback() {
        let mut y = 0;
        set_callback(move |x| {
            y += 1;
            x > y
        });

        assert!(invoke_callback(2));
        assert!(!invoke_callback(2));
        unset_callback();
    }
}
