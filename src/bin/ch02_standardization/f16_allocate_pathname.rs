use apue::result::PointerResult;
use libc::{malloc, sysconf, PATH_MAX, _SC_VERSION, _SC_XOPEN_VERSION};

fn path_alloc(sizep: &mut usize) -> *mut libc::c_void {
    let posix_version = unsafe { sysconf(_SC_VERSION) };
    let xsi_version = unsafe { sysconf(_SC_XOPEN_VERSION) };
    let mut size = PATH_MAX as usize;

    // Before POSIX.1-2001 && XPG, we aren't guaranteed that PATH_MAX includes the terminating null byte
    if posix_version < 200_112 && xsi_version < 4 {
        size += 1;
    }

    if let Ok(ptr) = unsafe { malloc(size).not_null() } {
        *sizep = size;
        ptr
    } else {
        panic!("malloc error for pathname");
    }
}

fn main() {
    let mut sizep = 0;

    let ptr = path_alloc(&mut sizep);
    println!("ptr = {:?}, sizep = {}", ptr, sizep);
}
