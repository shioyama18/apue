use libc::c_int;
use libc::FILE;

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
}
