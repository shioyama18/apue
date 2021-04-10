#![allow(non_upper_case_globals)]

use anyhow::Result;
use apue::bindings::{pthread_once, pthread_once_t};
use libc::{c_char, pthread_mutex_t, PTHREAD_MUTEX_INITIALIZER, PTHREAD_MUTEX_RECURSIVE};
use libc::{
    pthread_mutex_init, pthread_mutex_unlock, pthread_mutexattr_destroy, pthread_mutexattr_init,
    pthread_mutexattr_settype,
};
use std::ffi::CStr;
use std::mem;

static mut env_mutex: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
static mut init_done: pthread_once_t = pthread_once_t {
    __opaque: [0; 8],
    __sig: 0x30B1BCBA, // Taken from /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/pthread/pthread_impl.h
};

extern "C" {
    pub static environ: *const *const c_char;
}

unsafe extern "C" fn thread_init() {
    let mut attr = mem::zeroed();
    pthread_mutexattr_init(&mut attr);
    pthread_mutexattr_settype(&mut attr, PTHREAD_MUTEX_RECURSIVE);
    pthread_mutex_init(&mut env_mutex, &attr);
    pthread_mutexattr_destroy(&mut attr);
}

fn getenv(name: &str) -> Result<String> {
    unsafe {
        pthread_once(&mut init_done, Some(thread_init));

        let mut i = 0;
        while !(*environ.offset(i)).is_null() {
            let env = CStr::from_ptr(*environ.offset(i)).to_str()?;
            let pair = env.split("=").collect::<Vec<_>>();

            if pair[0] == name {
                pthread_mutex_unlock(&mut env_mutex);
                return Ok(pair[1].to_string());
            }

            i += 1;
        }

        pthread_mutex_unlock(&mut env_mutex);
    }

    Ok("".to_string())
}

fn main() {
    if let Ok(val) = getenv("HOME") {
        println!("HOME={}", val);
    }
}
