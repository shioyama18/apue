use anyhow::Result;
use apue::result::NumericResult;
use libc::{exit, fork, getppid, sleep, waitpid};
use std::ptr;

fn main() -> Result<()> {
    unsafe {
        let pid = fork().non_negative()?;

        // first child
        if pid == 0 {
            let pid = fork().non_negative()?;
            if pid > 0 {
                exit(0);
            }

            sleep(2);
            println!("second child, parent pid = {}", getppid());
            exit(0);
        }

        assert_eq!(waitpid(pid, ptr::null_mut(), 0), pid, "waitpid error");
    }
    Ok(())
}
