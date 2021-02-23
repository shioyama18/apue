use anyhow::Result;
use apue::c_str;
use apue::result::*;
use libc::{c_char, chdir, getcwd, malloc, sysconf, PATH_MAX, _SC_VERSION, _SC_XOPEN_VERSION};
use std::ffi::CString;

unsafe fn path_alloc(sizep: &mut usize) -> Result<*mut c_char> {
    // Before POSIX.1-2001 && XPG, we aren't guaranteed that PATH_MAX includes the terminating null byte
    let size = if sysconf(_SC_VERSION) < 200_112 && sysconf(_SC_XOPEN_VERSION) < 4 {
        PATH_MAX as usize + 1
    } else {
        PATH_MAX as usize
    };

    let ptr = malloc(size).not_null()? as *mut c_char;
    *sizep = size;
    Ok(ptr)
}

fn main() -> Result<()> {
    unsafe {
        chdir(c_str!("/var/spool")).non_negative()?;

        let mut size = 0;
        let ptr = path_alloc(&mut size)?;
        getcwd(ptr, size).not_null()?;
        println!("cwd = {}", CString::from_raw(ptr).into_string()?);
    }

    Ok(())
}
