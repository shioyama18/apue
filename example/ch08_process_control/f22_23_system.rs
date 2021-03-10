use anyhow::Result;
use apue::c_str;
use apue::result::NumericResult;
use apue::utils::pr_exit;
use errno::errno;
use libc::{_exit, c_char, c_int, execl, fork, waitpid, EINTR};

fn system(cmd: *const c_char) -> c_int {
    if cmd.is_null() {
        return 1;
    }

    let mut status = 0;

    unsafe {
        let pid = fork();
        match pid {
            pid if pid < 0 => status = -1,
            0 => {
                execl(
                    c_str!("/bin/sh"),
                    c_str!("sh"),
                    c_str!("-c"),
                    cmd,
                    0 as *const c_char,
                );
                _exit(127);
            }
            _ => {
                while waitpid(pid, &mut status, 0) < 0 {
                    if errno().0 != EINTR {
                        status = -1;
                        break;
                    }
                }
            }
        }
    }

    status
}

fn main() -> Result<()> {
    let status = system(c_str!("date")).non_negative()?;
    pr_exit(status);
    let status = system(c_str!("nosuchcommand")).non_negative()?;
    pr_exit(status);
    let status = system(c_str!("who; exit 44")).non_negative()?;
    pr_exit(status);

    Ok(())
}
