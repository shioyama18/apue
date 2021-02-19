use libc::{lseek, SEEK_CUR, STDIN_FILENO};

fn main() {
    if unsafe { lseek(STDIN_FILENO, 0, SEEK_CUR) == -1 } {
        println!("cannot seek");
    } else {
        println!("seek OK");
    }
}
