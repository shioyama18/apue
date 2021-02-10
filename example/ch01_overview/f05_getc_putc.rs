#![allow(dead_code)]

use apue::libc::{getc, putc, stdin, stdout};
use apue::result::NumericResult;
use libc::{ferror, EOF};
use std::io;

fn copy_stdin_stdout() -> io::Result<()> {
    unsafe {
        while let Ok(c) = getc(stdin).non_negative() {
            if putc(c, stdout) == EOF {
                return Err(io::Error::last_os_error());
            }
        }

        if ferror(stdin) != 0 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    copy_stdin_stdout()?;

    Ok(())
}
