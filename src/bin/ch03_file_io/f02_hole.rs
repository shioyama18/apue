use apue::{c_str, c_void};
use libc::{c_void, creat, lseek, mode_t, write, SEEK_SET, S_IRGRP, S_IROTH, S_IRUSR, S_IWUSR};

const BUF1: *const c_void = c_void!("abcdefghij");
const BUF2: *const c_void = c_void!("ABCDEFGHIJ");
const FILE_MODE: mode_t = S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH;

fn main() {
    unsafe {
        let fd = creat(c_str!("file.hole"), FILE_MODE);
        assert!(fd >= 0, "creat error: fd = {}", fd);
        assert_eq!(write(fd, BUF1, 10), 10, "buf1 write error");
        assert_ne!(lseek(fd, 16384, SEEK_SET), -1, "lseek error");
        assert_eq!(write(fd, BUF2, 10), 10, "buf2 write error");
    }
}
