#![allow(unused_imports)]
use libc::{c_char, c_void};

#[macro_export]
macro_rules! c_str {
    ($str:expr) => {{
        use std::ffi::CString;
        CString::new($str).unwrap().as_ptr()
    }};
}

#[macro_export]
macro_rules! c_void {
    ($str:expr) => {
        $str.as_ptr() as *mut c_void
    };
}

#[macro_export]
macro_rules! skip_fail {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    };
}
