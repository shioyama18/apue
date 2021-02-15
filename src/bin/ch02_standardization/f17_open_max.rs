use apue::result::NumericResult;
use errno::Errno;
use libc::{sysconf, _SC_OPEN_MAX};

const OPEN_MAX_GUESS: i64 = 256;

fn open_max() -> i64 {
    errno::set_errno(Errno(0));

    if let Ok(openmax) = unsafe { sysconf(_SC_OPEN_MAX).non_negative() } {
        openmax
    } else {
        if errno::errno().0 == 0 {
            OPEN_MAX_GUESS
        } else {
            panic!("sysconf error for _SC_OPEN_MAX");
        }
    }
}

fn main() {
    println!("{}", open_max());
}
