use apue::c_void;
use apue::libc::signal::signal;
use libc::{alarm, c_char, c_int, read, write, SIGALRM, SIG_ERR, STDIN_FILENO, STDOUT_FILENO};
use std::mem;

const MAXLINE: usize = 1024;

fn sig_alrm(_signo: c_int) {}

fn main() {
    unsafe {
        let line: [c_char; MAXLINE] = mem::zeroed();

        if signal(SIGALRM, sig_alrm as usize) == SIG_ERR {
            panic!("signal(SIGALRM) error");
        }

        alarm(10);

        let n = read(STDIN_FILENO, c_void!(line), MAXLINE);
        assert!(n >= 0, "read error");

        alarm(0);

        write(STDOUT_FILENO, c_void!(line), n as usize);
    }
}
