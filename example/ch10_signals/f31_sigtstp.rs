use apue::c_void;
use apue::result::NumericResult;
use errno::errno;
use libc::{
    c_char, c_int, getpid, kill, read, sigaddset, sigemptyset, signal, sigprocmask, write, SIGTSTP,
    SIG_DFL, SIG_IGN, SIG_UNBLOCK, STDIN_FILENO, STDOUT_FILENO,
};
use std::{mem, ptr};

const BUFFSIZE: usize = 1024;

fn sig_tstp(_signo: c_int) {
    println!("\nRunning SIGTSTP handler");
    unsafe {
        let mut mask = mem::zeroed();
        sigemptyset(&mut mask);
        sigaddset(&mut mask, SIGTSTP);
        sigprocmask(SIG_UNBLOCK, &mask, ptr::null_mut());

        // Reset disposition to default
        signal(SIGTSTP, SIG_DFL);

        // Send signal to ourself
        kill(getpid(), SIGTSTP);

        signal(SIGTSTP, sig_tstp as usize);
    }
}

fn main() {
    unsafe {
        let buf: [c_char; BUFFSIZE] = mem::zeroed();

        // Only catch SIGTSTP if we're running with a job-control shell
        if signal(SIGTSTP, SIG_IGN) == SIG_DFL {
            signal(SIGTSTP, sig_tstp as usize);
        }

        while let Ok(n) = read(STDIN_FILENO, c_void!(buf), BUFFSIZE).positive() {
            if write(STDOUT_FILENO, c_void!(buf), n as usize) != n {
                panic!("write error");
            }
        }

        assert!(errno().0 >= 0, "read error");
    }
}
