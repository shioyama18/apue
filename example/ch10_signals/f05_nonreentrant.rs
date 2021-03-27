use anyhow::Result;
use apue::c_str;
use apue::result::PointerResult;
use libc::{alarm, c_int, getpwnam, signal, SIGALRM};

unsafe extern "C" fn my_alarm(_signo: c_int) {
    println!("in signal handler");

    if let Err(e) = getpwnam(c_str!("root")).not_null() {
        panic!("getpwnam(root) error: {}", e);
    }

    alarm(1);
}

fn main() -> Result<()> {
    unsafe {
        signal(SIGALRM, my_alarm as usize);
        alarm(1);

        loop {
            let ptr = getpwnam(c_str!("nobody")).not_null()?;
            let pw_name = std::ffi::CStr::from_ptr((*ptr).pw_name).to_string_lossy();
            if pw_name != "nobody" {
                println!("return value corrupted! pw_name = {}", pw_name);
            }
        }
    }
}
