#![allow(unused_imports, dead_code)]

use anyhow::Result;
use apue::result::NumericResult;
use libc::{c_int, fcntl, flock, off_t};
use libc::{F_GETLK, F_RDLCK, F_SETLK, F_SETLKW, F_UNLCK, F_WRLCK, SEEK_CUR};
use std::mem;

unsafe fn lock_reg(fd: c_int, cmd: c_int, t: i16, offset: off_t, whence: i16, len: off_t) -> c_int {
    let mut lock: flock = mem::zeroed();
    lock.l_type = t; // F_RDLCK, F_WRLCK, F_UNLCK
    lock.l_start = offset; // byte offset, relative to l_whence
    lock.l_whence = whence; // SEEK_SET, SEEK_CUR, SEEK_END
    lock.l_len = len;
    fcntl(fd, cmd, &lock)
}

unsafe fn test_lock(fd: c_int, t: i16, offset: off_t, whence: c_int, len: off_t) -> Result<i32> {
    let mut lock: flock = mem::zeroed();
    lock.l_type = t;
    lock.l_start = offset;
    lock.l_whence = whence as i16;
    lock.l_len = len;

    fcntl(fd, F_GETLK, &lock).non_negative()?;

    if lock.l_type == F_UNLCK {
        Ok(0)
    } else {
        Ok(lock.l_pid)
    }
}

fn main() {
    unsafe {
        match test_lock(0, F_RDLCK, 0, SEEK_CUR, 0) {
            Ok(0) => println!("Region is not locked by another process"),
            Ok(pid) => println!("Region is locked by process {}", pid),
            Err(e) => eprintln!("{}", e),
        }
    }
}
