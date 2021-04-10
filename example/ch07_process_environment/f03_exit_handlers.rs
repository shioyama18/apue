use anyhow::Result;
use apue::c_str;
use apue::result::NumericResult;
use libc::{atexit, printf};

fn main() -> Result<()> {
    unsafe {
        atexit(my_exit2).zeroed()?;
        atexit(my_exit1).zeroed()?;
        atexit(my_exit1).zeroed()?;
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
