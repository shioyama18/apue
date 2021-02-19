use apue::{c_str, c_void};
use libc::{
    c_uint, c_void, lseek, open, read, remove, write, O_APPEND, O_CREAT, O_RDWR, O_TRUNC, SEEK_SET,
    S_IRGRP, S_IROTH, S_IRUSR, S_IWUSR,
};
use std::str;

const BUF1: *const c_void = c_void!("abcdefghij");
const BUF2: *const c_void = c_void!("ABCDEFGHIJ");
const FILE_MODE: c_uint = (S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH) as c_uint;

fn main() {
    unsafe {
        let fd = open(
            c_str!("tmp.txt"),
            O_APPEND | O_RDWR | O_CREAT | O_TRUNC,
            FILE_MODE,
        );
        assert!(fd >= 0, "creat error: fd = {}", fd);

        // Write 10 characters to file
        assert_eq!(write(fd, BUF1, 10), 10, "buf1 write error");

        // Read from beginning using lseek
        assert_ne!(lseek(fd, 0, SEEK_SET), -1, "lseek error");
        let buf = [0; 4096];
        assert_eq!(read(fd, buf.as_ptr() as *mut c_void, 10), 10);
        println!("first write: {}", str::from_utf8(&buf).unwrap());

        // Attempt to overwrite the first write
        assert_ne!(lseek(fd, 0, SEEK_SET), -1, "lseek error");
        assert_eq!(write(fd, BUF2, 10), 10, "buf2 write error");

        // Read again from beginning.
        // Second write was appended to first write
        assert_ne!(lseek(fd, 0, SEEK_SET), -1, "lseek error");
        assert_eq!(read(fd, buf.as_ptr() as *mut c_void, 20), 20);
        println!("second write: {}", str::from_utf8(&buf).unwrap());

        remove(c_str!("tmp.txt"));
    }
}
