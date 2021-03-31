use libc::c_void;
use libc::sleep;
use libc::{pthread_create, pthread_join};
use std::{fmt, mem, ptr};

#[derive(Default, Debug)]
struct Foo {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Foo {
    fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        Self { a, b, c, d }
    }
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r" structure at {:#x}
 Foo.a = {}
 Foo.b = {}
 Foo.c = {}
 Foo.d = {}",
            self as *const _ as usize, self.a, self.b, self.c, self.d
        )
    }
}

extern "C" fn thr_fn1(_arg: *mut c_void) -> *mut c_void {
    let foo = Box::new(Foo::new(1, 2, 3, 4));
    println!("thread 1:\n{}", foo);
    let fp = Box::into_raw(foo);
    fp as *mut c_void
}

fn main() {
    unsafe {
        let mut tid1 = mem::zeroed();
        let err = pthread_create(&mut tid1, ptr::null(), thr_fn1, ptr::null_mut());
        assert_eq!(err, 0, "can't create thread 1");

        let mut fp = mem::zeroed();
        let err = pthread_join(tid1, &mut fp);
        assert_eq!(err, 0, "can't join with thread 1");
        sleep(1);

        let foo: &mut Foo = &mut *(fp as *mut Foo);
        println!("parent:\n{}", foo);
    }
}
