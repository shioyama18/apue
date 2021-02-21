use apue::libc::stderr;
use errno::{set_errno, Errno};
use libc::{fprintf, perror, strerror, EACCES, ENOENT};
use std::{env, io};

fn main() -> io::Result<()> {
    unsafe {
        fprintf(
            stderr,
            "EACCES: %s\n".as_ptr() as *const i8,
            strerror(EACCES), // Create Error mesage from error code
        );

        // Set errno to read from perror
        set_errno(Errno(ENOENT));

        if let Some(program_name) = env::current_exe()?.to_str() {
            perror(program_name.as_ptr() as *const i8);
        }
    }
    Ok(())
}
