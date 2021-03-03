use anyhow::Result;
use apue::c_str;
use apue::result::NumericResult;
use libc::{atexit, printf};

fn main() -> Result<()> {
    unsafe {
        atexit(my_exit2).non_zero()?;
        atexit(my_exit1).non_zero()?;
        atexit(my_exit1).non_zero()?;
        println!("main is done");
    }
    Ok(())
}

extern "C" fn my_exit1() {
    unsafe {
        printf(c_str!("first exit handler\n"));
    }
}

extern "C" fn my_exit2() {
    unsafe {
        printf(c_str!("second exit handler\n"));
    }
}
