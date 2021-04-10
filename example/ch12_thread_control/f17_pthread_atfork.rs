#![allow(non_upper_case_globals)]

use anyhow::Result;
use apue::result::NumericResult;
use libc::{c_void, pthread_mutex_t, PTHREAD_MUTEX_INITIALIZER};
use libc::{
    fork, pause, pthread_atfork, pthread_create, pthread_mutex_lock, pthread_mutex_unlock, sleep,
};
use std::{mem, ptr};

static mut lock1: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
static mut lock2: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;

unsafe extern "C" fn prepare() {
    println!("preparing locks...");
    if pthread_mutex_lock(&mut lock1) != 0 {
        panic!("can't lock lock1 in prepare handler");
    }
    if pthread_mutex_lock(&mut lock2) != 0 {
        panic!("can't lock lock2 in prepare handler");
    }
}

unsafe extern "C" fn parent() {
    println!("parent unlocking locks...");
    if pthread_mutex_unlock(&mut lock1) != 0 {
        panic!("can't unlock lock1 in parent handler");
    }
    if pthread_mutex_unlock(&mut lock2) != 0 {
        panic!("can't unlock lock2 in parent handler");
    }
}

unsafe extern "C" fn child() {
    println!("child unlocking locks...");
    if pthread_mutex_unlock(&mut lock1) != 0 {
        panic!("can't unlock lock1 in child handler");
    }
    if pthread_mutex_unlock(&mut lock2) != 0 {
        panic!("can't unlock lock2 in child handler");
    }
}

extern "C" fn thr_fn(_arg: *mut c_void) -> *mut c_void {
    println!("thread started...");
    unsafe {
        pause();
    }
    0 as _
}

fn main() -> Result<()> {
    unsafe {
        pthread_atfork(Some(prepare), Some(parent), Some(child)).zeroed()?;

        let mut tid = mem::zeroed();
        pthread_create(&mut tid, ptr::null(), thr_fn, 0 as _).zeroed()?;
        sleep(2);
        println!("parent about to fork...");

        match fork() {
            pid if pid < 0 => panic!("fork failed"),
            0 => println!("child returned from fork"),
            _ => println!("parent returned from fork"),
        }
    }

    Ok(())
}
