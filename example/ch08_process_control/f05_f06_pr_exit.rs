#![allow(unconditional_panic)]

use anyhow::Result;
use apue::result::NumericResult;
use libc::{
    abort, c_int, exit, fork, raise, wait, SIGFPE, WCOREDUMP, WEXITSTATUS, WIFEXITED, WIFSIGNALED,
    WIFSTOPPED, WSTOPSIG, WTERMSIG,
};
use std::{mem, panic};

fn pr_exit(status: c_int) {
    if WIFEXITED(status) {
        println!("normal termination, exit status = {}", WEXITSTATUS(status));
    } else if WIFSIGNALED(status) {
        println!(
            "abnnormal termination, signal number = {}{}",
            WTERMSIG(status),
            if WCOREDUMP(status) {
                " (core file generated)"
            } else {
                ""
            }
        );
    } else if WIFSTOPPED(status) {
        println!("child stopped, signal number = {}", WSTOPSIG(status));
    }
}

fn main() -> Result<()> {
    // https://doc.rust-lang.org/beta/std/panic/struct.PanicInfo.html
    panic::set_hook(Box::new(|panic_info| {
        match panic_info.payload().downcast_ref::<String>() {
            Some(message) if message == "attempt to divide by zero" => unsafe {
                raise(SIGFPE);
            },
            _ => unimplemented!(),
        }
    }));

    unsafe {
        let mut status: c_int = mem::zeroed();

        let pid = fork().non_negative()?;
        if pid == 0 {
            // Child
            exit(7);
        }
        wait(&mut status).equals(pid)?;
        pr_exit(status);

        let pid = fork().non_negative()?;
        if pid == 0 {
            // Child
            abort();
        }
        wait(&mut status).equals(pid)?;
        pr_exit(status);

        let pid = fork().non_negative()?;
        if pid == 0 {
            status /= 0;
        }

        wait(&mut status).equals(pid)?;
        pr_exit(status);
    }

    Ok(())
}
