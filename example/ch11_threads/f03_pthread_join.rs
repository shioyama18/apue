use libc::c_void;
use libc::{pthread_create, pthread_join};
use std::{mem, ptr};

extern "C" fn thr_fn1(_arg: *mut c_void) -> *mut c_void {
    println!("thread 1 returning");
    1 as _
}

extern "C" fn thr_fn2(_arg: *mut c_void) -> *mut c_void {
    println!("thread 2 returning");
    2 as _
}

fn main() {
    unsafe {
        let mut tid1 = mem::zeroed();
        let err = pthread_create(&mut tid1, ptr::null(), thr_fn1, ptr::null_mut());
        assert_eq!(err, 0, "can't create thread 1");

        let mut tid2 = mem::zeroed();
        let err = pthread_create(&mut tid2, ptr::null(), thr_fn2, ptr::null_mut());
        assert_eq!(err, 0, "can't create thread 2");

        let mut tret = mem::zeroed();
        let err = pthread_join(tid1, &mut tret);
        assert_eq!(err, 0, "can't join with thread 1");
        println!("thread 1 exit code {}", tret as usize);

        let err = pthread_join(tid2, &mut tret);
        assert_eq!(err, 0, "can't join with thread 2");
        println!("thread 2 exit code {}", tret as usize);
    }
}
