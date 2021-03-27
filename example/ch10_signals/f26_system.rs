use apue::c_str;
use errno::errno;
use libc::{_exit, c_char, c_int, execl, fork, waitpid, EINTR};
use libc::{signal, SIGCHLD, SIGINT, SIG_ERR};

pub fn system(cmd: *const c_char) -> c_int {
    if cmd.is_null() {
        return 1;
    }

    let mut status = 0;

    unsafe {
        let pid = fork();
        match pid {
            pid if pid < 0 => status = -1,
            0 => {
                execl(
                    c_str!("/bin/sh"),
                    c_str!("sh"),
                    c_str!("-c"),
                    cmd,
                    0 as *const c_char,
                );
                _exit(127);
            }
            _ => {
                while waitpid(pid, &mut status, 0) < 0 {
                    if errno().0 != EINTR {
                        status = -1;
                        break;
                    }
                }
            }
        }
    }

    status
}

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
