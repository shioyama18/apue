use anyhow::Result;
use apue::c_str;
use apue::libc::{getc, stderr, stdin, stdout};
use apue::result::{NumericResult, PointerResult};
use libc::{fopen, fputs, getchar, FILE, _IOLBF, _IONBF};

#[cfg(target_os = "macos")]
fn pr_stdio(name: &str, fp: *mut FILE) {
    use apue::bindings::__sFILE;
    let fp = fp as *mut __sFILE;

    unsafe {
        let flags = (*fp)._flags as i32;

        let buffer_type = if flags & _IONBF != 0 {
            "unbuffered"
        } else if flags & _IOLBF != 0 {
            "line buffered"
        } else {
            "fully buffered"
        };

        let buffer_size = (*fp)._bf._size;

        println!(
            "stream = {}, {}, buffer size = {}",
            name, buffer_type, buffer_size,
        );
    }
}

#[cfg(not(target_os = "macos"))]
fn pr_stdio(name: &str, fp: *mut FILE) {
    unimplemented!();
}

fn main() -> Result<()> {
    unsafe {
        // use stdout
        fputs(c_str!("enter any character\n"), stdout);
        // use stdin
        getchar().non_negative()?;
        // use stderr
        fputs(c_str!("one line to standard error\n"), stderr);

        pr_stdio("stdin", stdin);
        pr_stdio("stdout", stdout);
        pr_stdio("stderr", stderr);

        let fp = fopen(c_str!("/etc/passwd"), c_str!("r")).not_null()?;
        getc(fp).non_negative()?;
        pr_stdio("/etc/passwd", fp);
    }

    Ok(())
}
