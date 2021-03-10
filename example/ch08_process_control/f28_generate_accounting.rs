//! Execute below command before running this program
//! $ sudo mkdir /var/account
//! $ sudo touch /var/account/acct
//! $ sudo accton /var/account/acct

use anyhow::Result;
use apue::c_str;
use apue::result::NumericResult;
use libc::{abort, c_char, execl, exit, fork, getpid, kill, sleep, SIGKILL};

fn main() -> Result<()> {
    unsafe {
        let pid = fork().non_negative()?;
        if pid != 0 {
            sleep(2);
            exit(2);
        }

        let pid = fork().non_negative()?;
        if pid != 0 {
            sleep(4);
            abort();
        }

        let pid = fork().non_negative()?;
        if pid != 0 {
            execl(
                c_str!("/bin/dd"),
                c_str!("dd"),
                c_str!("if=/etc/passwd"),
                c_str!("of=/dev/null"),
                0 as *const c_char,
            );
            exit(7);
        }

        let pid = fork().non_negative()?;
        if pid != 0 {
            sleep(8);
            exit(0);
        }

        sleep(6);
        kill(getpid(), SIGKILL);
        exit(6);
    }
}
