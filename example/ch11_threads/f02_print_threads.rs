use libc::{c_void, getpid, pthread_create, pthread_self, pthread_t, sleep};
use std::ptr;

static mut NTID: pthread_t = 0;

fn printids(s: &str) {
    unsafe {
        let pid = getpid();
        let tid = pthread_self();
        println!("{} pid {} tid {} ({:#x})", s, pid, tid, tid);
    }
}

extern "C" fn thr_fn(_arg: *mut c_void) -> *mut c_void {
    printids("new thread:");
    0 as _
}

fn main() {
    unsafe {
        let err = pthread_create(&mut NTID, ptr::null(), thr_fn, ptr::null_mut());
        assert_eq!(err, 0, "can't create thread");
        printids("main thread:");
        sleep(1);
    }
}
