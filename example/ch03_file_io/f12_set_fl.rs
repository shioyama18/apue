use apue::result::NumericResult;
use errno::errno;
use libc::{
    c_void, fcntl, read, size_t, write, F_GETFL, F_SETFL, O_SYNC, STDIN_FILENO, STDOUT_FILENO,
};
use std::io;

fn set_fl(fd: i32, flags: i32) -> io::Result<()> {
    let mut val = unsafe { fcntl(fd, F_GETFL, 0) };
    if val < 0 {
        return Err(io::Error::last_os_error());
    }

    val |= flags;

    if unsafe { fcntl(fd, F_SETFL, val) } < 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}

const BUFFSIZE: size_t = 4096;

fn main() -> io::Result<()> {
    set_fl(STDOUT_FILENO, O_SYNC)?;

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

    Ok(())
}
