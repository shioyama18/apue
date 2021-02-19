use apue::result::NumericResult;
use errno::errno;
use libc::{c_void, read, size_t, write, STDIN_FILENO, STDOUT_FILENO};

const BUFFSIZE: size_t = 4096;

fn main() {
    unsafe {
        let buf = [0; BUFFSIZE];

        while let Ok(n) = read(STDIN_FILENO, buf.as_ptr() as *mut c_void, BUFFSIZE).positive() {
            assert_eq!(
                write(STDOUT_FILENO, buf.as_ptr() as *const c_void, n as usize),
                n,
                "write error"
            );
        }

        assert!(errno().0 >= 0, "read error");
    }
}
