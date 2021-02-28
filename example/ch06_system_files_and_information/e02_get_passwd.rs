use anyhow::Result;
use apue::result::PointerResult;
use apue::{c_str, c_to_string};
use libc::getpwnam;
use std::ffi::CStr;

fn main() -> Result<()> {
    unsafe {
        let ptr = getpwnam(c_str!("root")).not_null()?;
        let password = match (*ptr).pw_passwd.not_null() {
            Err(_e) => "(null)".to_string(),
            Ok(pass) if CStr::from_ptr(pass).to_bytes()[0] == 0 => "(null)".to_string(),
            Ok(pass) => c_to_string!(pass).to_string(),
        };

        println!("pw_passwd = {}", password);
    }
    Ok(())
}
