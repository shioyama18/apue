use anyhow::Result;
use apue::c_str;
use apue::libc::stdout;
use apue::result::NumericResult;
use apue::utils::pr_exit;
use libc::{clock_t, setbuf, sysconf, system, times, tms, _SC_CLK_TCK};
use std::{mem, ptr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    cmds: Vec<String>,
}

unsafe fn pr_times(real: clock_t, tmsstart: &tms, tmsend: &tms) -> Result<()> {
    let clktck = sysconf(_SC_CLK_TCK).non_negative()? as f64;
    println!("  real: {:7.2}", real as f64 / clktck);
    println!(
        "  user: {:7.2}",
        (tmsend.tms_utime - tmsstart.tms_utime) as f64 / clktck
    );
    println!(
        "  sys:  {:7.2}",
        (tmsend.tms_stime - tmsstart.tms_stime) as f64 / clktck
    );
    println!(
        "  child user: {:7.2}",
        (tmsend.tms_cutime - tmsstart.tms_cutime) as f64 / clktck
    );
    println!(
        "  child sys:  {:7.2}",
        (tmsend.tms_cstime - tmsstart.tms_cstime) as f64 / clktck
    );

    Ok(())
}

unsafe fn do_cmd(cmd: String) -> Result<()> {
    println!("command: {}", cmd);
    let (mut tmsstart, mut tmsend): (tms, tms) = (mem::zeroed(), mem::zeroed());
    let start = times(&mut tmsstart);
    let status = system(c_str!(cmd)).non_negative()?;
    let end = times(&mut tmsend);

    pr_times(end - start, &tmsstart, &tmsend)?;
    pr_exit(status);

    Ok(())
}

fn main() -> Result<()> {
    let Opt { cmds } = Opt::from_args();

    unsafe {
        setbuf(stdout, ptr::null_mut());
        for cmd in cmds.into_iter() {
            do_cmd(cmd)?;
        }
    }

    Ok(())
}
