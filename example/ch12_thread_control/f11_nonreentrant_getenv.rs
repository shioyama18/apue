use anyhow::Result;
use libc::{c_char, strcpy};
use std::ffi::{CStr, CString};

const MAXSTRINGSZ: usize = 4096;
static mut ENVBUF: [c_char; MAXSTRINGSZ] = [0; MAXSTRINGSZ];

extern "C" {
    pub static environ: *const *const c_char;
}

fn getenv(name: &str) -> Result<()> {
    unsafe {
        let mut i = 0;
        while !(*environ.offset(i)).is_null() {
            let env = CStr::from_ptr(*environ.offset(i)).to_str()?;
            let pair = env.split("=").collect::<Vec<_>>();

            if pair[0] == name {
                strcpy(
                    ENVBUF.as_mut_ptr(),
                    CString::new(pair[1])?.as_c_str().as_ptr(),
                );
            }

            i += 1;
        }
    }

    Ok(())
}

fn main() {
    if getenv("HOME").is_ok() {
        unsafe {
            println!("{}", CStr::from_ptr(ENVBUF.as_ptr()).to_string_lossy());
        }
    }
}
