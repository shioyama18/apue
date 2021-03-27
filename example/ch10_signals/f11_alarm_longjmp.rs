use apue::bindings::{jmp_buf, longjmp, setjmp};
use apue::c_void;
use apue::libc::signal::signal;
use libc::{alarm, c_char, c_int, read, write, SIGALRM, SIG_ERR, STDIN_FILENO, STDOUT_FILENO};
use std::mem;

static mut ENV_ALRM: jmp_buf = [0; 37];

const MAXLINE: usize = 1024;

unsafe fn sig_alrm(_signo: c_int) {
    longjmp(ENV_ALRM.as_mut_ptr(), 1);
}

fn main() {
    unsafe {
        let line: [c_char; MAXLINE] = mem::zeroed();

        if signal(SIGALRM, sig_alrm as usize) == SIG_ERR {
            panic!("signal(SIGALRM) error");
        }

        if setjmp(ENV_ALRM.as_mut_ptr()) != 0 {
            eprintln!("read timeout");
        }

        alarm(10);

        let n = read(STDIN_FILENO, c_void!(line), MAXLINE);
        assert!(n >= 0, "read error");

        alarm(0);

        write(STDOUT_FILENO, c_void!(line), n as usize);
    }
}
