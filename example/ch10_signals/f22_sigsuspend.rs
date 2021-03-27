use anyhow::Result;
use apue::bindings::{sigprocmask, sigsuspend};
use apue::libc::signal::pr_mask;
use apue::result::NumericResult;
use libc::{
    c_int, sigaddset, sigemptyset, signal, SIGINT, SIGUSR1, SIG_BLOCK, SIG_ERR, SIG_SETMASK,
};
use std::{mem, ptr};

fn sig_int(_signo: c_int) -> Result<()> {
    pr_mask("in sig_int: ")?;
    Ok(())
}

fn main() -> Result<()> {
    unsafe {
        pr_mask("program start: ")?;

        if signal(SIGINT, sig_int as usize) == SIG_ERR {
            panic!("signal(SIGINT) error");
        }

        let mut waitmask = mem::zeroed();
        sigemptyset(&mut waitmask);
        sigaddset(&mut waitmask, SIGUSR1);

        let mut newmask = mem::zeroed();
        sigemptyset(&mut newmask);
        sigaddset(&mut newmask, SIGINT);

        let mut oldmask = mem::zeroed();
        // Block SIGINT ans save current signal mask
        sigprocmask(SIG_BLOCK, &newmask, &mut oldmask).non_negative()?;

        // Critical region
        pr_mask("in critical region: ")?;

        sigsuspend(&waitmask).equals(-1)?;

        pr_mask("after return from sigsuspend: ")?;

        // Unblock SIGINT
        sigprocmask(SIG_SETMASK, &oldmask, ptr::null_mut()).non_negative()?;

        pr_mask("program exit: ")?;
    }

    Ok(())
}
