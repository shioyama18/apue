#![allow(dead_code)]

use libc::{c_void, read, write, STDIN_FILENO, STDOUT_FILENO};
use std::io::{self, Read, Write};

const BUFFSIZE: usize = 4096;

unsafe fn copy_stdin_stdout_c() -> io::Result<()> {
    let buffer = [0; BUFFSIZE];

    loop {
        let n = read(STDIN_FILENO, buffer.as_ptr() as *mut c_void, BUFFSIZE);

        if n == 0 {
            return Ok(());
        }

        if n < 0 {
            return Err(io::Error::last_os_error());
        }

        if n != write(STDOUT_FILENO, buffer.as_ptr() as *const c_void, n as usize) {
            return Err(io::Error::last_os_error());
        }
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
    unsafe {
        copy_stdin_stdout_c()?;
    }

    Ok(())
}
