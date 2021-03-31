#![allow(clashing_extern_declarations)]

use crate::c_str;
use errno;
use libc::sigaddset;
use libc::{_exit, c_char, c_int, execl, fork, size_t, tm, waitpid, FILE};
use libc::{
    sigaction, sigemptyset, sigprocmask, EINTR, SIGCHLD, SIGINT, SIGQUIT, SIG_BLOCK, SIG_IGN,
    SIG_SETMASK,
};
use std::{mem, ptr};

pub mod signal;

extern "C" {
    pub fn getc(stream: *mut FILE) -> c_int;
    pub fn putc(n: c_int, stream: *mut FILE) -> c_int;

    #[cfg(target_os = "macos")]
    #[link_name = "__stdinp"]
    pub static mut stdin: *mut FILE;

    #[cfg(not(target_os = "macos"))]
    pub static mut stdin: *mut FILE;

    #[cfg(target_os = "macos")]
    #[link_name = "__stdoutp"]
    pub static mut stdout: *mut FILE;

    #[cfg(not(target_os = "macos"))]
    pub static mut stdout: *mut FILE;

    #[cfg(target_os = "macos")]
    #[link_name = "__stderrp"]
    pub static mut stderr: *mut FILE;

    #[cfg(not(target_os = "macos"))]
    pub static mut stderr: *mut FILE;

    pub fn strftime(
        s: *mut c_char,
        maxsize: size_t,
        format: *const c_char,
        timeptr: *const tm,
    ) -> size_t;
}

pub unsafe fn system(cmdstring: *const c_char) -> c_int {
    if cmdstring.is_null() {
        return 1;
    }

    let mut ignore: sigaction = mem::zeroed();
    ignore.sa_sigaction = SIG_IGN;
    sigemptyset(&mut ignore.sa_mask);
    ignore.sa_flags = 0;

    let mut saveintr = mem::zeroed();
    if sigaction(SIGINT, &ignore, &mut saveintr) < 0 {
        return -1;
    }
    if sigaction(SIGQUIT, &ignore, &mut saveintr) < 0 {
        return -1;
    }

    let mut chldmask = mem::zeroed();
    let mut savemask = mem::zeroed();
    sigemptyset(&mut chldmask);
    sigaddset(&mut chldmask, SIGCHLD);
    if sigprocmask(SIG_BLOCK, &chldmask, &mut savemask) < 0 {
        return -1;
    }

    let mut status = 0;
    let savequit = mem::zeroed();
    match fork() {
        pid if pid < 0 => status = -1,
        0 => {
            sigaction(SIGINT, &saveintr, ptr::null_mut());
            sigaction(SIGQUIT, &savequit, ptr::null_mut());
            sigprocmask(SIG_SETMASK, &savemask, ptr::null_mut());
            execl(
                c_str!("/bin/sh"),
                c_str!("sh"),
                c_str!("-c"),
                cmdstring,
                0 as *const c_char,
            );
            _exit(127);
        }
        pid => {
            while waitpid(pid, &mut status, 0) < 0 {
                if errno::errno().0 != EINTR {
                    status = -1;
                    break;
                }
            }
        }
    }

    status
}
