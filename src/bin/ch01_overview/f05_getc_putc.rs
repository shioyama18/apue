use apue::libc::{getc, putc, stdin, stdout};
use apue::result::NumericResult;
use libc::{ferror, EOF};
use std::io;

fn main() -> io::Result<()> {
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
