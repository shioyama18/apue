use anyhow::Result;
use apue::result::NumericResult;
use errno::errno;
use libc::{
    c_char, c_int, c_void, fork, getpgrp, getpid, getppid, kill, read, signal, sleep, tcgetpgrp,
    SIGHUP, SIGTSTP, STDIN_FILENO,
};

unsafe extern "C" fn sig_hup(_signo: c_int) {
    println!("SIGHUP received, pid = {}", getpid());
}

unsafe fn pr_ids(name: &str) {
    println!(
        "{}: pid = {}, ppid = {}, pgrp = {}, tpgrp = {}",
        name,
        getpid(),
        getppid(),
        getpgrp(),
        tcgetpgrp(STDIN_FILENO)
    );
}

fn main() -> Result<()> {
    unsafe {
        pr_ids("parent");
        let pid = fork().non_negative()?;

        match pid {
            0 => {
                pr_ids("child");
                signal(SIGHUP, sig_hup as *const () as usize);
                kill(getpid(), SIGTSTP);
                pr_ids("child");

                let c: c_char = std::mem::zeroed();
                if read(STDIN_FILENO, c as *mut c_void, 1) != 1 {
                    println!("read error {} on controlling TTY", errno().0);
                }
            }
            _ => {
                sleep(5);
            }
        }
    }

    Ok(())
}
