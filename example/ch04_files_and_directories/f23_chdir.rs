use apue::c_str;
use libc::chdir;
use std::process;

fn main() {
    unsafe {
        if chdir(c_str!("/tmp")) < 0 {
            eprintln!("chdir failed");
            process::exit(-1);
        }
    }

    println!("chdir to /tmp succeeded");
}
