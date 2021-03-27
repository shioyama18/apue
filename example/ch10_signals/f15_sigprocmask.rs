use anyhow::Result;
use apue::libc::signal::{sigaddset, sigismember};
use apue::result::NumericResult;
use libc::{
    c_int, sigemptyset, signal, sigpending, sigprocmask, sigset_t, sleep, SIGQUIT, SIG_BLOCK,
    SIG_DFL, SIG_ERR, SIG_SETMASK,
};
use std::{mem, ptr};

fn sig_quit(_signo: c_int) {
    println!("caught SIGQUIT");
    unsafe {
        if signal(SIGQUIT, SIG_DFL) == SIG_ERR {
            panic!("can't reset sigquit");
        }
    }
}

fn main() -> Result<()> {
    unsafe {
        let mut newmask: sigset_t = mem::zeroed();
        let mut oldmask: sigset_t = mem::zeroed();
        let mut pendmask: sigset_t = mem::zeroed();

        if signal(SIGQUIT, sig_quit as usize) == SIG_ERR {
            panic!("can't catch SIGQUIT");
        }

        sigemptyset(&mut newmask);
        sigaddset(&mut newmask, SIGQUIT);
        sigprocmask(SIG_BLOCK, &mut newmask, &mut oldmask).non_negative()?;

        sleep(5);

        sigpending(&mut pendmask).non_negative()?;
        if sigismember(&pendmask, SIGQUIT) {
            println!("SIGQUIT pending");
        }

        sigprocmask(SIG_SETMASK, &mut oldmask, ptr::null_mut()).non_negative()?;
        println!("SIGQUIT unblocked");

        sleep(5);
    }

    Ok(())
}
