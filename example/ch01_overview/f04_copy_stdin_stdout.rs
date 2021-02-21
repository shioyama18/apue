#![allow(dead_code)]

use apue::result::NumericResult;
use errno::errno;
use libc::{c_void, read, write, STDIN_FILENO, STDOUT_FILENO};
use std::io::{self, Read, Write};

const BUFFSIZE: usize = 4096;

fn copy_stdin_stdout_c() -> io::Result<()> {
    let buffer = [0; BUFFSIZE];

    unsafe {
        while let Ok(n) = read(STDIN_FILENO, buffer.as_ptr() as *mut c_void, BUFFSIZE).positive() {
            if n != write(STDOUT_FILENO, buffer.as_ptr() as *const c_void, n as usize) {
                return Err(io::Error::last_os_error());
            }
        }
    }

    if errno().0 != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn copy_stdin_stdout() -> io::Result<()> {
    let mut buffer = [0; BUFFSIZE];

    while let Ok(n) = io::stdin().read(&mut buffer) {
        if n == 0 {
            break;
        }

        io::stdout().write_all(&buffer)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    copy_stdin_stdout_c()?;

    Ok(())
}
