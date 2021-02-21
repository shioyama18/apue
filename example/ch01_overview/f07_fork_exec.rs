#![allow(dead_code)]

use apue::libc::stdin;
use apue::result::{NumericResult, PointerResult};
use libc::{c_char, execlp, fgets, fork, strlen, waitpid};
use std::io::{self, BufRead, Write};
use std::mem;
use std::process::Command;

const BUFFSIZE: usize = 4096;

/// C version of fork_exec
unsafe fn fork_exec_c() -> io::Result<()> {
    let mut buffer: [c_char; BUFFSIZE] = mem::zeroed();
    let mut status = 0;

    print_prompt()?;
    while let Ok(ptr) = fgets(buffer.as_ptr() as *mut i8, BUFFSIZE as i32, stdin).not_null() {
        if buffer[strlen(ptr) - 1] == '\n' as i8 {
            // Replace newline with null
            buffer[strlen(ptr) - 1] = 0;
        }

        let pid = fork();
        if pid < 0 {
            // Failed fork
            return Err(io::Error::last_os_error());
        }

        // Child process
        if pid == 0 {
            execlp(buffer.as_ptr(), buffer.as_ptr(), 0 as *const i8);

            // Failed exec
            return Err(io::Error::last_os_error());
        }

        if let Ok(_pid) = waitpid(pid, &mut status, 0).non_negative() {
            print_prompt()?;
        } else {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

/// Rust version of fork_exec
fn fork_exec() -> io::Result<()> {
    print_prompt()?;

    for line in io::stdin().lock().lines() {
        if let Ok(mut child) = Command::new("sh").arg("-c").arg(line?).spawn() {
            child.wait()?;
        } else {
            return Err(io::Error::last_os_error());
        }

        print_prompt()?;
    }

    Ok(())
}

fn print_prompt() -> io::Result<()> {
    print!("> ");
    io::stdout().flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    unsafe {
        fork_exec_c()?;
    }

    Ok(())
}
