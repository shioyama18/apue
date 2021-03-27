use anyhow::Result;
use apue::bindings::{sig_atomic_t, sigjmp_buf, siglongjmp, sigsetjmp};
use apue::libc::signal::{pr_mask, signal};
use libc::{alarm, c_int, pause, time, SIGALRM, SIGUSR1, SIG_ERR};
use std::ptr;

static mut JMPBUF: sigjmp_buf = [0; 38];
static mut CANJUMP: sig_atomic_t = 0;

fn sig_usr1(_signo: c_int) -> Result<()> {
    unsafe {
        if CANJUMP == 0 {
            return Ok(());
        }

        pr_mask("starting sig_usr1: ")?;
        alarm(3);
        let starttime = time(ptr::null_mut());
        loop {
            if time(ptr::null_mut()) > starttime + 5 {
                break;
            }
        }
        pr_mask("finishing sig_usr1: ")?;
        CANJUMP = 0;
        siglongjmp(JMPBUF.as_mut_ptr(), 1);
    }
    Ok(())
}

fn sig_alrm(_signo: c_int) -> Result<()> {
    pr_mask("in sig_alrm")?;
    Ok(())
}

fn main() -> Result<()> {
    if signal(SIGUSR1, sig_usr1 as usize) == SIG_ERR {
        panic!("signal(SIGUSR1) error");
    }
    if signal(SIGALRM, sig_alrm as usize) == SIG_ERR {
        panic!("signal(SIGUSR1) error");
    }

    pr_mask("starting main: ")?;

    unsafe {
        if sigsetjmp(JMPBUF.as_mut_ptr(), 1) != 0 {
            pr_mask("ending main: ")?;
            return Ok(());
        }

        CANJUMP = 1;

        loop {
            pause();
        }
    }
}
