use libc::pthread_join;
use libc::{c_int, c_void, pthread_attr_t, pthread_t};
use std::{mem, ptr};

#[link(name = "f05_pthread_cleanup")]
extern "C" {
    fn thr_fn1(arg: *mut c_void) -> *mut c_void;
    fn thr_fn2(arg: *mut c_void) -> *mut c_void;
}

extern "C" {
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: unsafe extern "C" fn(arg: *mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
}

fn main() {
    unsafe {
        let mut tid1 = mem::zeroed();
        let err = pthread_create(&mut tid1, ptr::null(), thr_fn1, 1 as _);
        assert_eq!(err, 0, "can't create thread 1");

        let mut tid2 = mem::zeroed();
        let err = pthread_create(&mut tid2, ptr::null(), thr_fn2, 1 as _);
        assert_eq!(err, 0, "can't create thread 2");

        let mut tret1 = mem::zeroed();
        let err = pthread_join(tid1, &mut tret1);
        assert_eq!(err, 0, "can't join with thread 1");
        println!("thread 1 exit code {}", tret1 as usize);

        let mut tret2 = mem::zeroed();
        let err = pthread_join(tid2, &mut tret2);
        assert_eq!(err, 0, "can't join with thread 2");
        println!("thread 2 exit code {}", tret2 as usize);
    }
}
