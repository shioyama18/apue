use apue::result::NumericResult;
use libc::{close, dup};
use std::process::{id, Command};
use std::str;

fn dup2(fd: i32, fd2: i32) -> i32 {
    if fd == fd2 {
        return fd2;
    }

    assert!(fd2 > fd, "Provided fd2 less than fd");

    unsafe {
        close(fd2);
    }

    while let Ok(new_fd) = unsafe { dup(fd).non_negative() } {
        if new_fd == fd2 {
            return new_fd;
        }
    }

    panic!("dup error");
}

fn main() {
    let fd = dup2(2, 5);
    println!("fd = {}", fd);

    let ls = Command::new("sh")
        .arg("-c")
        .arg(format!("lsof -a -p {}", id()))
        .output()
        .expect("failed to execute process");

    println!("{}", str::from_utf8(&ls.stdout).unwrap());
}
