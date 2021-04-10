#![allow(non_upper_case_globals)]

use anyhow::Result;
use apue::result::NumericResult;
use libc::{c_ulong, pthread_mutex_t, timespec};
use libc::{clock_gettime, nanosleep};
use libc::{
    pthread_mutex_lock, pthread_mutex_unlock, pthread_mutexattr_init, pthread_mutexattr_settype,
};
use libc::{CLOCK_REALTIME, PTHREAD_MUTEX_INITIALIZER, PTHREAD_MUTEX_RECURSIVE};
use std::{mem, ptr, thread, time};

static mut mutex: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;

struct ToInfo {
    to_fn: fn(c_ulong),
    to_arg: c_ulong,
    to_wait: timespec,
}

impl ToInfo {
    fn new(to_fn: fn(c_ulong), to_arg: c_ulong, to_wait: timespec) -> Self {
        Self {
            to_fn,
            to_arg,
            to_wait,
        }
    }
}

fn timeout_helper(tip: ToInfo) -> i32 {
    println!("start: timeout helper");
    unsafe {
        nanosleep(&tip.to_wait, ptr::null_mut());
        (tip.to_fn)(tip.to_arg);
    }
    println!("end: timeout helper");
    0
}

fn timeout(when: &mut timespec, func: fn(c_ulong), arg: c_ulong) {
    unsafe {
        let mut now = mem::zeroed();
        clock_gettime(CLOCK_REALTIME, &mut now);
        if when.tv_sec > now.tv_sec || when.tv_sec == now.tv_sec && when.tv_nsec > now.tv_nsec {
            let mut to_wait: timespec = mem::zeroed();
            to_wait.tv_sec = when.tv_sec - now.tv_sec;
            if when.tv_nsec >= now.tv_nsec {
                to_wait.tv_nsec = when.tv_nsec - now.tv_nsec;
            } else {
                to_wait.tv_sec -= 1;
                to_wait.tv_nsec = 1_000_000_000 - now.tv_nsec + when.tv_nsec;
            }
            let tip = ToInfo::new(func, arg, to_wait);
            thread::spawn(|| timeout_helper(tip));
            thread::sleep(time::Duration::from_secs(1));
            return;
        }

        // We get here if:
        // a) when <= now
        // b) thread failed to spawn
        func(arg);
    }
}

fn retry(_arg: c_ulong) {
    println!("start: retry");
    unsafe {
        pthread_mutex_lock(&mut mutex);
        println!("  mutex locked!");
        pthread_mutex_unlock(&mut mutex);
        println!("  mutex unlocked!");
    }
    println!("end: retry");
}

fn main() -> Result<()> {
    unsafe {
        let mut attr = mem::zeroed();
        pthread_mutexattr_init(&mut attr).zeroed()?;
        pthread_mutexattr_settype(&mut attr, PTHREAD_MUTEX_RECURSIVE).zeroed()?;
        pthread_mutex_lock(&mut mutex);

        let mut when = mem::zeroed();
        let condition = true;
        if condition {
            clock_gettime(CLOCK_REALTIME, &mut when);
            when.tv_sec += 1;
            timeout(&mut when, retry, 0);
        }

        pthread_mutex_unlock(&mut mutex);
    }

    Ok(())
}
