use anyhow::Result;
use apue::c_str;
use apue::libc::strftime;
use apue::result::{NumericResult, PointerResult};
use libc::{c_char, localtime, time};
use std::ffi::CStr;
use std::{io, mem, ptr};

fn main() -> Result<()> {
    unsafe {
        let caltime = time(ptr::null_mut()).non_negative()?;
        let tm = localtime(&caltime).not_null()?;

        let mut line: [c_char; 4096] = mem::zeroed();
        if strftime(line.as_mut_ptr(), 4096, c_str!("%a %b %d %X %Z %Y"), tm) == 0 {
            return Err(io::Error::last_os_error().into());
        }

        println!("{}", CStr::from_ptr(line.as_ptr()).to_string_lossy());
    }

    Ok(())
}
