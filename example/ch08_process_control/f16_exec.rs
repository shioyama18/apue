//! run `cargo intall --path <path_to_apue>` to install echoall

use anyhow::Result;
use apue::c_str;
use apue::result::NumericResult;
use libc::{c_char, execle, execlp, fork, waitpid};
use std::{env, ptr};

fn main() -> Result<()> {
    let cargo_home = env::var("CARGO_HOME")?;

    unsafe {
        let pid = fork().non_negative()?;

        if pid == 0 {
            execle(
                c_str!(format!("{}/bin/echoall", cargo_home)),
                c_str!("echoall"),
                c_str!("myarg1"),
                c_str!("MY_ARG2"),
                0 as *const c_char,
                [
                    c_str!("USER=unknown"),
                    c_str!("PATH=/tmp"),
                    0 as *const c_char,
                ]
                .as_ptr(),
            )
            .non_negative()?;
        }
        waitpid(pid, ptr::null_mut(), 0).non_negative()?;

        let pid = fork().non_negative()?;
        if pid == 0 {
            execlp(
                c_str!("echoall"),
                c_str!("echoall"),
                c_str!("only 1 arg"),
                0 as *const c_char,
            );
        }
        waitpid(pid, ptr::null_mut(), 0).non_negative()?;
    }

    Ok(())
}
