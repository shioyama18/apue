use anyhow::Result;
use apue::result::PointerResult;
use apue::{c_str, c_void};
use libc::{c_char, c_int, fclose, fflush, fmemopen, fprintf, fseek, memset, strlen, SEEK_SET};
use std::ffi::CStr;
use std::mem;

const BSZ: usize = 48;

macro_rules! buf_to_str {
    ($buf:expr) => {
        CStr::from_ptr($buf.as_ptr()).to_str()?
    };
}

fn main() -> Result<()> {
    unsafe {
        let mut buf: [c_char; BSZ] = mem::zeroed();
        memset(c_void!(buf), 'a' as c_int, BSZ - 2);
        buf[BSZ - 2] = '\0' as c_char;
        buf[BSZ - 1] = 'X' as c_char;
        let fp = fmemopen(c_void!(buf), BSZ, c_str!("w+")).not_null()?;

        // Results in empty string since fmemopen places a null byte at beginning of buffer
        println!("initial buffer contents: {}", buf_to_str!(buf));
        fprintf(fp, c_str!("hello, world"));

        #[cfg(not(target_os = "macos"))]
        println!("before fflush: {}", buf_to_str!(buf));

        fflush(fp);
        println!("after fflush: {}", buf_to_str!(buf));
        println!("len of string in buf = {}", strlen(buf.as_ptr()));

        memset(c_void!(buf), 'b' as c_int, BSZ - 2);
        buf[BSZ - 2] = '\0' as c_char;
        buf[BSZ - 1] = 'X' as c_char;
        fprintf(fp, c_str!("hello, world"));
        fseek(fp, 0, SEEK_SET);
        println!("after fseek: {}", buf_to_str!(buf));
        println!("len of string in buf = {}", strlen(buf.as_ptr()));

        memset(c_void!(buf), 'c' as c_int, BSZ - 2);
        buf[BSZ - 2] = '\0' as c_char;
        buf[BSZ - 1] = 'X' as c_char;
        fprintf(fp, c_str!("hello, world"));
        fclose(fp);
        // This code does not print correctly on MacOS since it appends NULL even if the file is closed
        #[cfg(not(target_os = "macos"))]
        {
            println!("after fclose: {}", buf_to_str!(buf));
            println!("len of string in buf = {}", strlen(buf.as_ptr()));
        }
    }
    Ok(())
}
