use libc::{getgid, getuid};
use std::io;

fn main() -> io::Result<()> {
    unsafe {
        println!("uid = {}, gid = {}", getuid(), getgid());
    }

    Ok(())
}
