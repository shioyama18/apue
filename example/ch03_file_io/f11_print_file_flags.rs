use libc::{
    fcntl, F_GETFL, O_ACCMODE, O_APPEND, O_FSYNC, O_NONBLOCK, O_RDONLY, O_RDWR, O_SYNC, O_WRONLY,
};
use std::{env, process};

fn main() {
    let fd = env::args()
        .nth(1)
        .and_then(|fd| fd.parse::<i32>().ok())
        .expect("usage: cargo run <descriptor#>");
    let val = unsafe { fcntl(fd, F_GETFL, 0) };

    assert!(val >= 0, "fcntl error for fd {}", fd);

    match val & O_ACCMODE {
        O_RDONLY => print!("read only"),
        O_WRONLY => print!("write only"),
        O_RDWR => print!("read write"),
        _ => {
            eprintln!("unknown access mode");
            process::exit(-1);
        }
    }

    if val & O_APPEND != 0 {
        print!(", append");
    }
    if val & O_NONBLOCK != 0 {
        print!(", nonblocking");
    }
    if val & O_SYNC != 0 {
        print!(", synchronous writes");
    }
    if val & O_FSYNC != 0 && O_FSYNC != O_SYNC && val & O_FSYNC != 0 {
        print!(", synchronous writes");
    }

    println!();
}
