use anyhow::Result;
use apue::c_str;
use apue::libc::stdout;
use apue::result::PointerResult;
use libc::{fgets, fputs, rewind, tmpfile, tmpnam, L_tmpnam};
use std::ffi::{CStr, CString};
use std::ptr;

const MAXLINE: usize = 4096;

fn main() -> Result<()> {
    unsafe {
        println!(
            "first temp name = {:?}",
            CStr::from_ptr(tmpnam(ptr::null_mut()))
        );

        let name = CString::from(Vec::with_capacity(L_tmpnam as usize)).into_raw();
        tmpnam(name);
        println!("second temp name = {:?}", CStr::from_ptr(name));

        let fp = tmpfile().not_null()?;
        let line = CString::from(Vec::with_capacity(MAXLINE)).into_raw();
        fputs(c_str!("one line of output\n"), fp);
        rewind(fp);
        fgets(line, MAXLINE as i32, fp).not_null()?;
        fputs(line, stdout);
    }

    Ok(())
}
