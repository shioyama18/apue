use anyhow::Result;
use apue::bindings::{sig_atomic_t, sigprocmask, sigsuspend};
use apue::result::NumericResult;
use libc::{
    c_int, sigaddset, sigemptyset, signal, SIGINT, SIGQUIT, SIG_BLOCK, SIG_ERR, SIG_SETMASK,
};
use std::{mem, ptr};

static mut QUITFLAG: sig_atomic_t = 0;

fn sig_int(signo: c_int) {
    if signo == SIGINT {
        println!("interrupt");
    }
    if signo == SIGQUIT {
        unsafe {
            QUITFLAG = 1;
        }
    }
}

fn main() -> Result<()> {
    unsafe {
        if signal(SIGINT, sig_int as usize) == SIG_ERR {
            panic!("signal(SIGINT) error");
        }

        if signal(SIGQUIT, sig_int as usize) == SIG_ERR {
            panic!("signal(SIGQUIT) error");
        }

        let mut zeromask = mem::zeroed();
        let mut newmask = mem::zeroed();
        let mut oldmask = mem::zeroed();
        sigemptyset(&mut zeromask);
        sigemptyset(&mut newmask);
        sigaddset(&mut newmask, SIGQUIT);

        // Block SIGQUIT and save current signal
        sigprocmask(SIG_BLOCK, &newmask, &mut oldmask).non_negative()?;

        while QUITFLAG == 0 {
            sigsuspend(&zeromask);
        }

        // SIGQUIT has been caught and is now blocked
        QUITFLAG = 0;

        // Reset signal mask which unblocks SIGQUIT
        sigprocmask(SIG_SETMASK, &oldmask, ptr::null_mut()).non_negative()?;
    }

    Ok(())
}
