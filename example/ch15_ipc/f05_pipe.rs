use anyhow::Result;
use apue::result::NumericResult;
use libc::STDOUT_FILENO;
use libc::{close, fork, pipe, read, write};

fn main() -> Result<()> {
    unsafe {
        let mut fd = [0; 2];
        pipe(fd.as_mut_ptr()).non_negative()?;
        let pid = fork().non_negative()?;
        match pid {
            0 => {
                close(fd[1]);
                let mut line = [0; 1024];
                let n = read(fd[0], line.as_mut_ptr() as _, 1024);
                write(STDOUT_FILENO, line.as_ptr() as _, n as usize);
            }
            _ => {
                close(fd[0]);
                write(fd[1], "hello world\n".as_ptr() as _, 12);
            }
        }
    }

    Ok(())
}
