#![no_main]

use libc::{c_char, printf};

#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) {
    unsafe {
        printf("hello, world\n".as_ptr() as *const c_char);
    }
}
