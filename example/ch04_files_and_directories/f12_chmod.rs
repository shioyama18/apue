//! This program needs to be run as root in order to add S_ISGID to "foo"

use anyhow::Result;
use apue::c_str;
use apue::result::NumericResult;
use libc::{chmod, stat, S_IRGRP, S_IROTH, S_IRUSR, S_ISGID, S_IWUSR, S_IXGRP};
use std::mem;

fn main() -> Result<()> {
    unsafe {
        let mut statbuf: stat = mem::zeroed();
        stat(c_str!("foo"), &mut statbuf).non_negative()?;
        chmod(c_str!("foo"), (statbuf.st_mode & !S_IXGRP) | S_ISGID).non_negative()?;
        chmod(c_str!("bar"), S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH).non_negative()?;
    }

    Ok(())
}
