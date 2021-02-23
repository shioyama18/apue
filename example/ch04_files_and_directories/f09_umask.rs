use anyhow::Result;
use apue::c_str;
use apue::result::NumericResult;
use libc::{creat, mode_t, umask, S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR};

const RWRWRW: mode_t = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;

fn main() -> Result<()> {
    unsafe {
        umask(0);
        creat(c_str!("foo"), RWRWRW).non_negative()?;

        umask(S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH);
        creat(c_str!("bar"), RWRWRW).non_negative()?;
    }
    Ok(())
}
