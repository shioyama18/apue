use anyhow::Result;
use apue::c_str;
use apue::result::NumericResult;
use libc::{open, unlink, O_RDWR};
use std::{thread, time};

fn main() -> Result<()> {
    unsafe {
        open(c_str!("tempfile"), O_RDWR).non_negative()?;
        unlink(c_str!("tempfile")).non_negative()?;
    }

    println!("file unlinked");
    thread::sleep(time::Duration::from_secs(15));
    println!("done");
    Ok(())
}
