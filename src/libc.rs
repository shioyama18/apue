use libc::FILE;
use libc::{c_char, c_int, size_t, tm};

extern "C" {
    pub fn getc(stream: *mut FILE) -> c_int;
    pub fn putc(n: c_int, stream: *mut FILE) -> c_int;

    #[cfg(target_os = "macos")]
    #[link_name = "__stdinp"]
    pub static mut stdin: *mut FILE;

    #[cfg(not(target_os = "macos"))]
    pub static mut stdin: *mut FILE;

    #[cfg(target_os = "macos")]
    #[link_name = "__stdoutp"]
    pub static mut stdout: *mut FILE;

    #[cfg(not(target_os = "macos"))]
    pub static mut stdout: *mut FILE;

    #[cfg(target_os = "macos")]
    #[link_name = "__stderrp"]
    pub static mut stderr: *mut FILE;

    #[cfg(not(target_os = "macos"))]
    pub static mut stderr: *mut FILE;

    pub fn strftime(
        s: *mut c_char,
        maxsize: size_t,
        format: *const c_char,
        timeptr: *const tm,
    ) -> size_t;
}
