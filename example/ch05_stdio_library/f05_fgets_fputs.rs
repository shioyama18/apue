use anyhow::Result;
use apue::libc::{stdin, stdout};
use apue::result::{NumericResult, PointerResult};
use libc::{ferror, fgets, fputs};

const MAXLINE: usize = 4096;

fn main() -> Result<()> {
    unsafe {
        let mut buf = [0; MAXLINE];
        while let Ok(ptr) = fgets(buf.as_mut_ptr(), MAXLINE as i32, stdin).not_null() {
            fputs(ptr, stdout).non_negative()?;
        }

        ferror(stdin).zeroed()?;
    }

    Ok(())
}
