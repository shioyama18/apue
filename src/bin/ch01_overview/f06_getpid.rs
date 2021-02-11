use libc::getpid;
use std::process;

fn main() {
    println!("rust: hello world from process ID {}", process::id());

    unsafe {
        println!("c: hello world from process ID {}", getpid());
    }
}
