use anyhow::Result;
use apue::result::NumericResult;
use errno::errno;
use libc::{c_char, close, mkstemp, stat, unlink, ENOENT};
use std::ffi::{CStr, CString};
use std::mem;

fn make_temp(template: *mut c_char) -> Result<()> {
    unsafe {
        let fd = mkstemp(template).non_negative()?;
        println!("temp name = {:?}", CStr::from_ptr(template));
        close(fd);

        let mut sbuf: stat = mem::zeroed();
        match stat(template, &mut sbuf).non_negative() {
            Ok(_) => {
                println!("file exists");
                unlink(template);
            }
            Err(e) => {
                if errno().0 == ENOENT {
                    println!("file doesn't exist");
                } else {
                    return Err(e.into());
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("trying to create temp file...");
    make_temp(CString::new("/tmp/dirXXXXXX")?.into_raw())?;

    Ok(())
}
