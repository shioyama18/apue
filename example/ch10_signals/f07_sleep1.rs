use libc::{alarm, c_int, c_uint, pause, signal, SIGALRM, SIG_ERR};

extern "C" fn sig_alrm(_signo: c_int) {}

fn sleep1(seconds: c_uint) -> c_uint {
    unsafe {
        if signal(SIGALRM, sig_alrm as usize) == SIG_ERR {
            return seconds;
        }

        alarm(seconds);
        pause();
        alarm(0)
    }
}

fn main() {
    println!("sleeping...");
    sleep1(5);
    println!("done.");
}
