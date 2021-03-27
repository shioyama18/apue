use apue::c_str;
use apue::libc::system;
use libc::{c_int, signal, SIGCHLD, SIGINT, SIG_ERR};

fn sig_int(_signo: c_int) {
    println!("caught SIGINT");
}

fn sig_chld(_signo: c_int) {
    println!("caught SIGCHLD");
}

fn main() {
    unsafe {
        if signal(SIGINT, sig_int as usize) == SIG_ERR {
            panic!("signal(SIGINT) error");
        }
        if signal(SIGCHLD, sig_chld as usize) == SIG_ERR {
            panic!("signal(SIGCHLD) error");
        }
        if system(c_str!("/bin/ed")) < 0 {
            panic!("system() error");
        }
    }
}
