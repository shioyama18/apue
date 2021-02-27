use anyhow::Result;
use apue::libc::{getc, putc, stdin, stdout};
use apue::result::NumericResult;
use libc::ferror;

fn main() -> Result<()> {
    unsafe {
        while let Ok(c) = getc(stdin).non_negative() {
            putc(c, stdout).non_negative()?;
        }

        if ferror(stdin) != 0 {
            eprintln!("input error");
            std::process::exit(-1);
        }
    }

    Ok(())
}
