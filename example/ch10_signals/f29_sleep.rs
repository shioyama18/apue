use apue::bindings::sigsuspend;
use libc::{
    alarm, c_int, c_uint, sigaction, sigaddset, sigdelset, sigemptyset, sigprocmask, SIGALRM,
    SIG_BLOCK, SIG_SETMASK,
};
use std::{mem, ptr};

fn sig_alrm(_signo: c_int) {}

unsafe fn sleep(seconds: c_uint) -> c_uint {
    let mut newact: sigaction = mem::zeroed();
    let mut oldact: sigaction = mem::zeroed();

    // Set handler and save previous information
    newact.sa_sigaction = sig_alrm as usize;
    sigemptyset(&mut newact.sa_mask);
    newact.sa_flags = 0;
    sigaction(SIGALRM, &newact, &mut oldact);

    let mut newmask = mem::zeroed();
    let mut oldmask = mem::zeroed();

    // Block SIGALRM and save current signal mask
    sigemptyset(&mut newmask);
    sigaddset(&mut newmask, SIGALRM);
    sigprocmask(SIG_BLOCK, &newmask, &mut oldmask);

    alarm(seconds);
    let mut suspmask = oldmask;

    // Unblock SIGALRM
    sigdelset(&mut suspmask, SIGALRM);

    // Wait for any signal
    sigsuspend(&suspmask);

    // Some signal has been caught, SIGALRM is now blocked
    let unslept = alarm(0);

    // Reset previous action
    sigaction(SIGALRM, &oldact, ptr::null_mut());

    // Reset signal mask, which unblocks SIGALRM
    sigprocmask(SIG_SETMASK, &oldmask, ptr::null_mut());

    unslept
}

fn main() {
    unsafe {
        println!("sleeping...");
        println!("Done! Slept for {} seconds", 5 - sleep(5));
    }
}
