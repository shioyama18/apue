use anyhow::Result;
use apue::libc::{putc, stdout};
use apue::result::NumericResult;
use libc::{fork, setbuf, usleep};
use std::ptr;

unsafe fn charatatime(s: &str) {
    setbuf(stdout, ptr::null_mut());
    for c in s.chars() {
        putc(c as i32, stdout);
        usleep(10);
    }
}

fn main() -> Result<()> {
    unsafe {
        let pid = fork().non_negative()?;
        if pid == 0 {
            charatatime("output from child\n");
        } else {
            charatatime("output from parent\n");
        }
    }

    Ok(())
}
