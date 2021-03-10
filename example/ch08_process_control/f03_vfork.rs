#![allow(unused_assignments)]
use libc::{_exit, getpid, pid_t};

extern "C" {
    fn vfork() -> pid_t;
}

// external variable in initialized area;
static mut GLOBVAR: i32 = 6;

fn main() {
    // variable kept on stack
    let mut var = 88;

    unsafe {
        println!("before fork");
        let pid = vfork();
        assert!(pid >= 0, "fork error");

        // Child
        if pid == 0 {
            GLOBVAR += 1;
            var += 1;
            _exit(0);
        }

        println!("pid = {}, glob = {}, var = {}", getpid(), GLOBVAR, var);
    }
}
