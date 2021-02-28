use apue::libc::strftime;
use apue::{c_str, c_to_string};
use libc::{c_char, localtime, time, time_t};
use std::mem;

fn main() {
    unsafe {
        let mut t: time_t = mem::zeroed();
        time(&mut t);
        let tmp = localtime(&t);

        let mut buf1: [c_char; 16] = mem::zeroed();
        if strftime(
            buf1.as_mut_ptr(),
            16,
            c_str!("time and date: %r, %a %b %d, %Y"),
            tmp,
        ) == 0
        {
            println!("buffer length 16 is too small");
        } else {
            println!("{}", c_to_string!(buf1.as_ptr()));
        }

        let mut buf2: [c_char; 64] = mem::zeroed();
        if strftime(
            buf2.as_mut_ptr(),
            64,
            c_str!("time and date: %r, %a %b %d, %Y"),
            tmp,
        ) == 0
        {
            println!("buffer length 64 is too small");
        } else {
            println!("{}", c_to_string!(buf2.as_ptr()));
        }
    }
}
