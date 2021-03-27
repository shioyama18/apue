use crate::bindings::NSIG;
use crate::result::NumericResult;
use anyhow::Result;
use errno::{errno, set_errno, Errno};
use libc::{
    c_int, fflush, getpid, kill, sigaction, sigemptyset, sigfillset, sigprocmask, sigset_t,
};
use libc::{
    EINVAL, SA_RESTART, SIGABRT, SIGALRM, SIGINT, SIGQUIT, SIGUSR1, SIG_DFL, SIG_ERR, SIG_IGN,
    SIG_SETMASK,
};
use std::{mem, ptr};

fn sigbad(signo: c_int) -> bool {
    signo <= 0 || signo >= NSIG as i32
}

pub fn sigaddset(set: &mut sigset_t, signo: c_int) -> c_int {
    if sigbad(signo) {
        set_errno(Errno(EINVAL));
        return -1;
    }

    *set |= 1 << (signo - 1);

    0
}

pub fn sigdelset(set: &mut sigset_t, signo: c_int) -> c_int {
    if sigbad(signo) {
        set_errno(Errno(EINVAL));
        return -1;
    }

    *set &= !(1 << (signo - 1));

    0
}

pub fn sigismember(set: &sigset_t, signo: c_int) -> bool {
    if sigbad(signo) {
        set_errno(Errno(EINVAL));
        return false;
    }

    *set & 1 << (signo - 1) != 0
}

pub fn pr_mask(s: &str) -> Result<()> {
    let errno_save = errno();
    unsafe {
        let mut sigset: sigset_t = mem::zeroed();

        sigprocmask(0, ptr::null(), &mut sigset).non_negative()?;

        let mut buf = vec![s];

        if sigismember(&sigset, SIGINT) {
            buf.push("SIGINT");
        }
        if sigismember(&sigset, SIGQUIT) {
            buf.push("SIGQUIT");
        }
        if sigismember(&sigset, SIGUSR1) {
            buf.push("SIGUSR1");
        }
        if sigismember(&sigset, SIGALRM) {
            buf.push("SIGALRM");
        }

        println!("{}", buf.join(" "));
    }

    set_errno(Errno::from(errno_save));
    Ok(())
}

pub fn signal(signo: c_int, func: usize) -> usize {
    unsafe {
        let mut act: sigaction = mem::zeroed();
        let mut oact: sigaction = mem::zeroed();
        act.sa_sigaction = func;
        sigemptyset(&mut act.sa_mask);
        act.sa_flags = 0;

        if signo != SIGALRM {
            act.sa_flags |= SA_RESTART;
        }

        if sigaction(signo, &act, &mut oact) < 0 {
            SIG_ERR
        } else {
            oact.sa_sigaction
        }
    }
}

pub fn abort() {
    unsafe {
        let mut action = mem::zeroed();
        sigaction(SIGABRT, ptr::null(), &mut action);
        if action.sa_sigaction == SIG_IGN {
            action.sa_sigaction = SIG_DFL;
            sigaction(SIGABRT, &action, ptr::null_mut());
        }
        if action.sa_sigaction == SIG_DFL {
            fflush(ptr::null_mut());
        }

        let mut mask = mem::zeroed();
        sigfillset(&mut mask);
        sigdelset(&mut mask, SIGABRT);
        sigprocmask(SIG_SETMASK, &mask, ptr::null_mut());
        kill(getpid(), SIGABRT);

        fflush(ptr::null_mut());
        action.sa_sigaction = SIG_DFL;
        sigaction(SIGABRT, &action, ptr::null_mut());
        sigprocmask(SIG_SETMASK, &mask, ptr::null_mut());
        kill(getpid(), SIGABRT);
    }
}
