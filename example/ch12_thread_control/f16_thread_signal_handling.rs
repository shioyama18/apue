#![allow(non_upper_case_globals)]

use anyhow::Result;
use apue::result::NumericResult;
use libc::{
    c_void, pthread_cond_t, pthread_mutex_t, sigset_t, PTHREAD_COND_INITIALIZER,
    PTHREAD_MUTEX_INITIALIZER,
};
use libc::{
    pthread_cond_signal, pthread_cond_wait, pthread_create, pthread_mutex_lock,
    pthread_mutex_unlock, pthread_sigmask,
};
use libc::{sigaddset, sigemptyset, sigprocmask, sigwait, SIGINT, SIGQUIT, SIG_BLOCK, SIG_SETMASK};
use std::{mem, ptr};

static mut quitflag: bool = false;
static mut mask: sigset_t = 0;
static mut lock: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
static mut waitloc: pthread_cond_t = PTHREAD_COND_INITIALIZER;

extern "C" fn thr_fn(_arg: *mut c_void) -> *mut c_void {
    unsafe {
        let mut signo = mem::zeroed();
        loop {
            if sigwait(&mask, &mut signo) != 0 {
                panic!("sigwait failed")
            }
            match signo {
                SIGINT => println!("\ninterrupt"),
                SIGQUIT => {
                    pthread_mutex_lock(&mut lock);
                    quitflag = true;
                    pthread_mutex_unlock(&mut lock);
                    pthread_cond_signal(&mut waitloc);
                    return 0 as _;
                }
                _ => {
                    panic!("unexpected signal {}", signo);
                }
            }
        }
    }
}

fn main() -> Result<()> {
    unsafe {
        sigemptyset(&mut mask);
        sigaddset(&mut mask, SIGINT);
        sigaddset(&mut mask, SIGQUIT);

        let mut oldmask = mem::zeroed();
        pthread_sigmask(SIG_BLOCK, &mask, &mut oldmask).zeroed()?;
        let mut tid = mem::zeroed();
        pthread_create(&mut tid, ptr::null(), thr_fn, 0 as _).zeroed()?;

        while !quitflag {
            pthread_cond_wait(&mut waitloc, &mut lock);
        }
        pthread_mutex_unlock(&mut lock);

        sigprocmask(SIG_SETMASK, &oldmask, ptr::null_mut()).non_negative()?;
    }

    Ok(())
}
