use anyhow::Result;
use apue::bindings::NZERO;
use apue::libc::stdout;
use apue::result::NumericResult;
use errno;
use libc::{exit, fork, gettimeofday, nice, setbuf, timeval};
use std::{io, mem, ptr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    adj: Option<i32>,
}

fn main() -> Result<()> {
    let Opt { adj } = Opt::from_args();
    let adj = adj.unwrap_or(0);
    let nzero = NZERO as i32;

    unsafe {
        setbuf(stdout, ptr::null_mut());
        println!("NZERO = {}", nzero);
        let mut end: timeval = mem::zeroed();
        gettimeofday(&mut end, ptr::null_mut());
        end.tv_sec += 10; // run for 10 seconds

        let pid = fork().non_negative()?;
        let s = match pid {
            0 => {
                println!(
                    "current nice value in child is {}, adjusting by {}",
                    nice(0) + nzero,
                    adj
                );
                errno::set_errno(errno::Errno(0));
                let ret = nice(adj);
                if ret == -1 && errno::errno().0 != 0 {
                    return Err(io::Error::last_os_error().into());
                }
                println!("now child nice value is {}", ret + nzero);
                "child"
            }
            _ => {
                println!("current nice value in parent is {}", nice(0) + nzero);
                "parent"
            }
        };

        let mut count = 0;
        let mut tv: timeval = mem::zeroed();

        loop {
            count += 1;
            gettimeofday(&mut tv, ptr::null_mut());
            if tv.tv_sec >= end.tv_sec && tv.tv_usec >= end.tv_usec {
                println!("{} count = {}", s, count);
                exit(0);
            }
        }
    }
}
