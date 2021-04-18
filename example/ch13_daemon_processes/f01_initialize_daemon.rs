use anyhow::Result;
use apue::c_str;
use apue::result::NumericResult;
use libc::{close, dup, exit, fork, open, setsid, umask, O_RDWR};
use libc::{getrlimit, RLIMIT_NOFILE, RLIM_INFINITY};
use libc::{openlog, syslog, LOG_CONS, LOG_DAEMON, LOG_ERR};
use libc::{sigaction, sigemptyset, SIGHUP, SIG_IGN};
use std::{env, io, mem, path, ptr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    cmd: String,
}

unsafe fn daemonize(cmd: &str) -> Result<()> {
    // Clear file creation mask
    umask(0);

    // Get max number of fd
    let mut rl = mem::zeroed();
    getrlimit(RLIMIT_NOFILE, &mut rl).non_negative()?;

    // Become session leader to lose controlling TTY
    match fork() {
        pid if pid < 0 => return Err(io::Error::last_os_error().into()),
        0 => {
            setsid();
        }
        _ => exit(0),
    }

    // Ensure future opens won't allocate controlling TTYs
    let mut sa: sigaction = mem::zeroed();
    sa.sa_sigaction = SIG_IGN;
    sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = 0;
    sigaction(SIGHUP, &sa, ptr::null_mut()).non_negative()?;
    let pid = fork().non_negative()?;
    if pid != 0 {
        exit(0);
    }

    // Change current working directory to the root so
    // we won't prevent file systems from being unmounted
    let root = path::Path::new("/");
    env::set_current_dir(&root)?;

    // Close all open file descriptors
    if rl.rlim_max == RLIM_INFINITY {
        rl.rlim_max = 1024;
    }
    for i in 0..rl.rlim_max {
        close(i as i32);
    }

    // Attach file descriptors 0, 1, and 2 to /dev/null
    let fd0 = open(c_str!("/dev/null"), O_RDWR);
    let fd1 = dup(0);
    let fd2 = dup(0);

    // Initialize the log file
    openlog(c_str!(cmd), LOG_CONS, LOG_DAEMON);
    if fd0 != 0 || fd1 != 1 || fd2 != 2 {
        syslog(
            LOG_ERR,
            c_str!("unexpected file descriptors %d %d %d"),
            fd0,
            fd1,
            fd2,
        );
        exit(1);
    }

    Ok(())
}

fn main() {
    let Opt { cmd } = Opt::from_args();

    unsafe {
        if let Err(e) = daemonize(&cmd) {
            eprintln!("{}", e);
        }
    }
}
