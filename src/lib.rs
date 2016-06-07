extern crate libc;
mod c;

use self::libc::{c_void, c_int};
use std::mem;

pub struct Callback<'a> {
    #[allow(dead_code)] // used by C
    callback: Box<Box<FnMut(i32) -> i32 + 'a>>,
    c_handle: *const c::Callback,
}

extern fn callback_handler(x: c_int, arg: *mut c_void) -> c_int {
    let closure: &mut Box<FnMut(i32) -> i32> = unsafe { mem::transmute(arg) };
    closure(x as i32) as c_int
}

impl<'a> Callback<'a> {
    pub fn new<F>(callback: F) -> Option<Self>
        where F: FnMut(i32) -> i32 + 'a {
        let callback: Box<Box<FnMut(i32) -> i32 + 'a>> = Box::new(Box::new(callback));
        let ptr = &*callback as *const Box<_> as *mut c_void;

        match unsafe{c::add_callback(Some(callback_handler), ptr).as_ref()} {
            None => None,
            Some(c_handle) => Some(Callback{callback: callback, c_handle: c_handle}),
        }
    }
}

impl<'a> Drop for Callback<'a> {
    fn drop(&mut self) {
        unsafe {c::remove_callback(self.c_handle)};
    }
}

pub fn invoke_callbacks(x: i32) -> i32 {
    unsafe {
        c::invoke_callbacks(x as c_int)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_callback() {
        let mut y = 0;
        let callback = Callback::new(move |x| {
            y += 1;
            x - y
        });

        assert!(invoke_callbacks(2) == 1);
        assert!(invoke_callbacks(2) == 0);
        drop(callback);
    }

    #[test]
    fn test_multiple_callbacks() {
        let mut y = 0;
        let callback1 = Callback::new(move |x| {
            y += 1;
            x - y
        });
        let callback2 = Callback::new(|x| {
            x % 3
        });

        assert!(invoke_callbacks(2) == 3);
        assert!(invoke_callbacks(2) == 2);
        drop(callback1);
        assert!(invoke_callbacks(2) == 2);
        drop(callback2);
    }
}
