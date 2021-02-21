use apue::result::NumericResult;
use errno::{set_errno, Errno};
use libc::{pathconf, sysconf, EINVAL, _PC_MAX_CANON, _SC_ARG_MAX};
use std::{env, io, process};

enum Conf<'a> {
    SysConf,
    PathConf { path: &'a str },
}

use Conf::*;

fn pr_conf(conf: Conf, mesg: &str, name: i32) {
    set_errno(Errno(0));

    let val = match conf {
        SysConf => unsafe { sysconf(name) },
        PathConf { path } => unsafe { pathconf(path.as_ptr() as *const i8, name) },
    };

    match val.positive() {
        Ok(val) => println!("{}{}", mesg, val),
        Err(_e) => {
            let errno = errno::errno().0;
            if errno != 0 {
                if errno == EINVAL {
                    println!("{}(not supported)", mesg);
                } else {
                    match conf {
                        SysConf => eprintln!("sysconf error"),
                        _ => eprintln!("pathconf error"),
                    }
                    process::exit(-1);
                }
            } else {
                println!("{}(no limit)", mesg);
            }
        }
    }
}

fn main() -> io::Result<()> {
    pr_conf(SysConf, "ARG_MAX = ", _SC_ARG_MAX);
    if let Some(ref path) = env::args().nth(1) {
        pr_conf(PathConf { path }, "MAX_CANON = ", _PC_MAX_CANON);
    } else {
        eprintln!("usage: ./f14_limits <dirname>");
        process::exit(-1);
    }

    Ok(())
}
