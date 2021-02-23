use anyhow::Result;
use apue::result::NumericResult;
use apue::skip_fail;
use libc::{close, futimens, open, stat, timespec, O_RDWR, O_TRUNC};
use std::ffi::CString;
use std::mem;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = CString::new))]
    files: Vec<CString>,
}

fn main() -> Result<()> {
    let Opt { files } = Opt::from_args();
    for file in files.into_iter() {
        unsafe {
            let mut statbuf: stat = mem::zeroed();

            // fetch current times
            skip_fail!(stat(file.as_ptr(), &mut statbuf).non_negative());

            // truncate file
            let fd = skip_fail!(open(file.as_ptr(), O_RDWR | O_TRUNC).non_negative());

            let times: [timespec; 2] = [
                timespec {
                    tv_sec: statbuf.st_atime,
                    tv_nsec: statbuf.st_atime_nsec,
                },
                timespec {
                    tv_sec: statbuf.st_mtime,
                    tv_nsec: statbuf.st_mtime_nsec,
                },
            ];

            // reset times
            futimens(fd, times.as_ptr() as *const timespec).non_negative()?;

            close(fd);
        }
    }
    Ok(())
}
