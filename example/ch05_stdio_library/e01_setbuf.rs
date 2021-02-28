use anyhow::Result;
use apue::libc::stdin;
use apue::result::NumericResult;
use libc::{c_char, setvbuf, strlen, FILE, _IOFBF, _IONBF};
use std::ptr;

unsafe fn setbuf(stream: *mut FILE, buf: *mut c_char) -> Result<()> {
    if buf.is_null() {
        setvbuf(stream, buf, _IONBF, 0).non_negative()?;
    } else {
        setvbuf(stream, buf, _IOFBF, strlen(buf)).non_negative()?;
    }

    Ok(())
}

fn main() -> Result<()> {
    unsafe {
        setbuf(stdin, ptr::null_mut())?;
    }

    Ok(())
}
