#![allow(non_camel_case_types, non_snake_case)]
use apue::bindings::{heapsort, random, srandom};
use libc::{c_int, c_long, c_uint, c_void, pthread_cond_t, pthread_mutex_t};
use libc::{gettimeofday, pthread_create};
use libc::{PTHREAD_COND_INITIALIZER, PTHREAD_MUTEX_INITIALIZER};
use std::{cmp, mem, ptr};

const NTHR: usize = 8;
const NUMNUM: usize = 8_000_000;
const TNUM: usize = NUMNUM / NTHR;

static mut NUMS: [i64; NUMNUM] = [0; NUMNUM];
static mut SNUMS: [i64; NUMNUM] = [0; NUMNUM];
static mut B: pthread_barrier_t = pthread_barrier_t {
    mutex: PTHREAD_MUTEX_INITIALIZER,
    cond: PTHREAD_COND_INITIALIZER,
    count: 0,
    tripCount: 0,
};

type pthread_barrierattr_t = c_int;

#[repr(C)]
pub struct pthread_barrier_t {
    mutex: pthread_mutex_t,
    cond: pthread_cond_t,
    count: c_int,
    tripCount: c_int,
}

extern "C" {
    pub fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const pthread_barrierattr_t,
        count: c_uint,
    ) -> c_int;
    pub fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;
    pub fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;

}

extern "C" fn complong(arg1: *const c_void, arg2: *const c_void) -> c_int {
    unsafe {
        let arg1 = arg1 as *const c_long;
        let arg2 = arg2 as *const c_long;

        match (*arg1).cmp(&*arg2) {
            cmp::Ordering::Less => -1,
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => 1,
        }
    }
}

extern "C" fn thr_fn(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let idx = arg as isize;

        heapsort(
            NUMS.as_mut_ptr().offset(idx) as *mut c_void,
            TNUM as u64,
            mem::size_of::<i64>() as u64,
            Some(complong),
        );
        pthread_barrier_wait(&mut B);
        0 as *mut c_void
    }
}

unsafe fn merge() {
    let mut idx = [0; NTHR];
    for i in 0..NTHR {
        idx[i] = i * TNUM;
    }

    let mut minidx = 0;
    for sidx in 0..NUMNUM {
        let mut num = std::i64::MAX;
        for i in 0..NTHR {
            if idx[i] < (i + 1) * TNUM && NUMS[idx[i]] < num {
                num = NUMS[idx[i]];
                minidx = i;
            }
        }
        SNUMS[sidx] = NUMS[idx[minidx]];
        idx[minidx] += 1;
    }
}

fn main() {
    unsafe {
        srandom(1);
        for i in 0..NUMNUM {
            NUMS[i] = random();
        }

        let mut start = mem::zeroed();
        gettimeofday(&mut start, ptr::null_mut());
        pthread_barrier_init(&mut B, ptr::null(), NTHR as u32 + 1);
        let mut tid = mem::zeroed();
        for i in 0..NTHR {
            let err = pthread_create(&mut tid, ptr::null(), thr_fn, (i * TNUM) as *mut c_void);
            assert_eq!(err, 0, "can't create thread");
        }
        pthread_barrier_wait(&mut B);
        merge();

        let mut end = mem::zeroed();
        gettimeofday(&mut end, ptr::null_mut());
        let startusec = start.tv_sec * 1_000_000 + start.tv_usec as i64;
        let endusec = end.tv_sec * 1_000_000 + end.tv_usec as i64;
        let elapsed = (endusec - startusec) as f64 / 1_000_000f64;
        println!("sort took {:.04} seconds", elapsed);
        for i in 0..NUMNUM {
            println!("{}", SNUMS[i]);
        }
    }
}
