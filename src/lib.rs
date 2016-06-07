extern crate libc;
mod c;

use self::libc::{c_void, c_int};
use std::mem;

extern fn callback_handler(x: c_int, arg: *mut c_void) -> c_int {
    let closure: &mut Box<FnMut(i32) -> i32> = unsafe { mem::transmute(arg) };
    closure(x as i32) as c_int
}

pub fn add_callback<F>(callback: F) -> *const c::Callback
    where F: FnMut(i32) -> i32, F: 'static {
    let cb: Box<Box<FnMut(i32) -> i32>> = Box::new(Box::new(callback));
    let ptr = Box::into_raw(cb) as * mut _;
    unsafe {
        c::add_callback(Some(callback_handler), ptr)
    }
}

pub fn invoke_callbacks(x: i32) -> i32 {
    unsafe {
        c::invoke_callbacks(x as c_int)
    }
}

pub fn remove_callback(callback: *const c::Callback) {
    let ptr = unsafe { c::remove_callback(callback) };
    // drop the callback
    let _: Box<Box<FnMut(i32) -> i32>> = unsafe { Box::from_raw(ptr as *mut _) };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_callback() {
        let mut y = 0;
        let callback = add_callback(move |x| {
            y += 1;
            x - y
        });

        assert!(invoke_callbacks(2) == 1);
        assert!(invoke_callbacks(2) == 0);
        remove_callback(callback);
    }

    #[test]
    fn test_multiple_callbacks() {
        let mut y = 0;
        let callback1 = add_callback(move |x| {
            y += 1;
            x - y
        });
        let callback2 = add_callback(|x| {
            x % 3
        });

        assert!(invoke_callbacks(2) == 3);
        assert!(invoke_callbacks(2) == 2);
        remove_callback(callback1);
        assert!(invoke_callbacks(2) == 2);
        remove_callback(callback2);
    }
}
