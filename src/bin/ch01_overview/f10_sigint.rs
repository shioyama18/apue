#![allow(dead_code)]

use apue::libc::stdin;
use apue::result::{NumericResult, PointerResult};
use libc::{c_char, execlp, fgets, fork, signal, strlen, waitpid, SIGINT, SIG_ERR};
use std::io::{self, Write};
use std::mem;

const BUFFSIZE: usize = 4096;

fn print_prompt() -> io::Result<()> {
    print!("> ");
    io::stdout().flush()?;
    Ok(())
}

fn sigint_handler() {
    println!("interrupt");
    print_prompt().unwrap();
}

unsafe fn fork_exec() -> io::Result<()> {
    let mut buffer: [c_char; BUFFSIZE] = mem::zeroed();
    let mut status = 0;

    // Register signal handler
    if signal(SIGINT, sigint_handler as usize) == SIG_ERR {
        return Err(io::Error::last_os_error());
    }

    print_prompt()?;
    while let Ok(ptr) = fgets(buffer.as_ptr() as *mut i8, BUFFSIZE as i32, stdin).not_null() {
        if buffer[strlen(ptr) - 1] == '\n' as i8 {
            // Replace newline with null
            buffer[strlen(ptr) - 1] = 0;
        }

        let pid = fork();

        // Failed fork
        if pid < 0 {
            return Err(io::Error::last_os_error());
        }

        // Child process
        if pid == 0 {
            execlp(buffer.as_ptr(), buffer.as_ptr(), 0 as *const i8);

            // Failed exec
            return Err(io::Error::last_os_error());
        }

        // Parent process
        if let Ok(_pid) = waitpid(pid, &mut status, 0).non_negative() {
            print_prompt()?;
        } else {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    unsafe {
        fork_exec()?;
    }

    Ok(())
}
