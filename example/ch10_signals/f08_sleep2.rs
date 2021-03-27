use apue::bindings::{jmp_buf, longjmp, setjmp};
use libc::{alarm, c_uint, pause, signal, SIGALRM, SIGINT, SIG_ERR};

static mut ENV_ALRM: jmp_buf = [0; 37];

unsafe extern "C" fn sig_alrm(_signo: i32) {
    longjmp(ENV_ALRM.as_ptr() as *mut i32, 1);
}

unsafe extern "C" fn sig_int(_signo: i32) {
    println!("sig_int starting...");
    let mut k = 0usize;
    for i in 0..300000 {
        for j in 0..4000 {
            k += i * j;
        }
    }
    println!("sig_int finished: k = {}", k);
}

unsafe fn sleep2(seconds: c_uint) -> c_uint {
    if signal(SIGALRM, sig_alrm as usize) == SIG_ERR {
        return seconds;
    }

    if setjmp(ENV_ALRM.as_ptr() as *mut i32) == 0 {
        alarm(seconds);
        pause();
    }

    alarm(0)
}

fn main() {
    unsafe {
        if signal(SIGINT, sig_int as usize) == SIG_ERR {
            panic!("signal(SIGINT) error");
        }
        let unslept = sleep2(5);
        println!("sleep2 returned: {}", unslept);
    }
}
