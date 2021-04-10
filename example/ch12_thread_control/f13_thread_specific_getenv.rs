#![allow(non_upper_case_globals)]

use anyhow::Result;
use apue::bindings::{pthread_once, pthread_once_t};
use libc::{c_char, pthread_key_t, pthread_mutex_t, PTHREAD_MUTEX_INITIALIZER};
use libc::{
    c_void, free, malloc, pthread_getspecific, pthread_key_create, pthread_mutex_lock,
    pthread_mutex_unlock, pthread_setspecific, strcpy,
};
use std::ffi::{CStr, CString};
use std::{io, ptr};

const MAXSTRINGSZ: usize = 4096;
static mut key: pthread_key_t = 0;
static mut env_mutex: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
static mut init_done: pthread_once_t = pthread_once_t {
    __opaque: [0; 8],
    __sig: 0x30B1BCBA, // Taken from /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/pthread/pthread_impl.h
};

extern "C" {
    pub static environ: *const *const c_char;
}

unsafe extern "C" fn thread_init() {
    pthread_key_create(&mut key, Some(free));
}

fn getenv(name: &str) -> Result<*const c_char> {
    unsafe {
        pthread_once(&mut init_done, Some(thread_init));
        pthread_mutex_lock(&mut env_mutex);
        let mut envbuf = pthread_getspecific(key) as *mut c_char;
        if envbuf.is_null() {
            envbuf = malloc(MAXSTRINGSZ) as *mut c_char;
            if envbuf.is_null() {
                pthread_mutex_unlock(&mut env_mutex);
                return Err(io::Error::last_os_error().into());
            }
            pthread_setspecific(key, envbuf as *const c_void);
        }
        let mut i = 0;
        while !(*environ.offset(i)).is_null() {
            let env = CStr::from_ptr(*environ.offset(i)).to_str()?;
            let pair = env.split("=").collect::<Vec<_>>();

            if pair[0] == name {
                strcpy(envbuf, CString::new(pair[1])?.as_c_str().as_ptr());

                pthread_mutex_unlock(&mut env_mutex);
                return Ok(envbuf);
            }

            i += 1;
        }

        pthread_mutex_unlock(&mut env_mutex);
    }

    Ok(ptr::null())
}

fn main() {
    if let Ok(val) = getenv("HOME") {
        unsafe {
            println!("HOME={}", CStr::from_ptr(val).to_string_lossy());
        }
    }
}
