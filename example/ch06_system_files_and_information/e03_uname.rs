use anyhow::Result;
use apue::c_to_string;
use apue::result::NumericResult;
use libc::{uname, utsname};
use std::mem;

fn main() -> Result<()> {
    unsafe {
        let mut name: utsname = mem::zeroed();
        uname(&mut name).non_negative()?;
        println!(
            "{} {} {} {} {}",
            c_to_string!(name.sysname.as_ptr()),
            c_to_string!(name.nodename.as_ptr()),
            c_to_string!(name.release.as_ptr()),
            c_to_string!(name.version.as_ptr()),
            c_to_string!(name.machine.as_ptr())
        );
    }

    Ok(())
}
