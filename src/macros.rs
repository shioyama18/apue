#[macro_export]
macro_rules! c_str {
    ($str:expr) => {{
        use std::ffi::CString;
        CString::new($str).unwrap().as_ptr()
    }};
}

#[macro_export]
macro_rules! c_void {
    ($str:expr) => {{
        use libc::c_void;
        $str.as_ptr() as *mut c_void
    }};
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

#[macro_export]
macro_rules! c_to_string {
    ($str:expr) => {{
        use std::ffi::CStr;
        CStr::from_ptr($str).to_string_lossy()
    }};
}
