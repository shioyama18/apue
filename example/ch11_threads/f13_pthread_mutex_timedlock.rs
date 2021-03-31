use apue::c_str;
use apue::libc::strftime;
use libc::{c_int, pthread_mutex_t, timespec};
use libc::{
    clock_gettime, localtime, pthread_mutex_lock, strerror, CLOCK_REALTIME,
    PTHREAD_MUTEX_INITIALIZER,
};
use std::ffi::CStr;
use std::mem;

const BUFSIZE: usize = 64;

#[link(name = "f13_pthread_mutex_timedlock")]
extern "C" {
    fn pthread_mutex_timedlock(mutex: *mut pthread_mutex_t, abs_timeout: *const timespec) -> c_int;
}

fn main() {
    unsafe {
        let mut lock = PTHREAD_MUTEX_INITIALIZER;
        pthread_mutex_lock(&mut lock);
        println!("mutex is locked");

        let mut tout = mem::zeroed();
        clock_gettime(CLOCK_REALTIME, &mut tout);
        let tmp = localtime(&tout.tv_sec);
        let mut buf = [0; BUFSIZE];
        strftime(buf.as_mut_ptr(), BUFSIZE, c_str!("%r"), tmp);
        println!(
            "current time is {}",
            CStr::from_ptr(buf.as_ptr()).to_string_lossy()
        );
        tout.tv_sec += 10;
        let err = pthread_mutex_timedlock(&mut lock, &tout);
        clock_gettime(CLOCK_REALTIME, &mut tout);
        let tmp = localtime(&tout.tv_sec);
        strftime(buf.as_mut_ptr(), BUFSIZE, c_str!("%r"), tmp);
        println!(
            "the time is now {}",
            CStr::from_ptr(buf.as_ptr()).to_string_lossy()
        );

        if err == 0 {
            println!("mutex locked again");
        } else {
            println!(
                "can't lock mutex again: {}",
                CStr::from_ptr(strerror(err)).to_string_lossy()
            );
        }
    }
}
