use anyhow::Result;
use apue::result::NumericResult;
use libc::{
    c_int, getrlimit, rlimit, RLIMIT_AS, RLIMIT_CORE, RLIMIT_CPU, RLIMIT_DATA, RLIMIT_FSIZE,
    RLIMIT_MEMLOCK, RLIMIT_NPROC, RLIMIT_RSS, RLIMIT_STACK, RLIM_INFINITY,
};
use std::mem;

macro_rules! doit {
    ($name:expr) => {
        pr_limits(stringify!($name), $name)?;
    };
}

fn pr_limits(name: &str, resource: c_int) -> Result<()> {
    unsafe {
        let mut limit: rlimit = mem::zeroed();
        getrlimit(resource, &mut limit).non_negative()?;

        let rlim_cur = if limit.rlim_cur == RLIM_INFINITY {
            "(infinite)".to_string()
        } else {
            format!("{}", limit.rlim_cur)
        };

        let rlim_max = if limit.rlim_max == RLIM_INFINITY {
            "(infinite)".to_string()
        } else {
            format!("{}", limit.rlim_max)
        };

        println!("{} {} {}", name, rlim_cur, rlim_max);
    }
    Ok(())
}

fn main() -> Result<()> {
    doit!(RLIMIT_AS);
    doit!(RLIMIT_CORE);
    doit!(RLIMIT_CPU);
    doit!(RLIMIT_DATA);
    doit!(RLIMIT_FSIZE);
    doit!(RLIMIT_MEMLOCK);
    doit!(RLIMIT_NPROC);
    doit!(RLIMIT_RSS);
    doit!(RLIMIT_STACK);

    Ok(())
}
