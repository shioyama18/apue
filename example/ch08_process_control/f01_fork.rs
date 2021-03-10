use apue::c_void;
use libc::{fork, getpid, write, STDOUT_FILENO};
use std::{thread, time};

// external variable in initialized area;
static mut GLOBVAR: i32 = 6;

fn main() {
    // variable kept on stack
    let mut var = 88;
    let buf = "a write to stdout\n".as_bytes();

    unsafe {
        assert_eq!(
            write(STDOUT_FILENO, c_void!(buf), buf.len()),
            buf.len() as isize,
            "write error"
        );

        println!("before fork");
        let pid = fork();
        assert!(pid >= 0, "fork error");

        if pid == 0 {
            // child
            GLOBVAR += 1;
            var += 1;
        } else {
            // parent
            thread::sleep(time::Duration::new(2, 0));
        }

        println!("pid = {}, glob = {}, var = {}", getpid(), GLOBVAR, var);
    }
}
