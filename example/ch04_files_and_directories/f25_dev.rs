use anyhow::Result;
use apue::result::NumericResult;
use apue::skip_fail;
use libc::{dev_t, stat, S_IFBLK, S_IFCHR, S_IFMT};
use std::ffi::CString;
use std::mem;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = CString::new))]
    pathnames: Vec<CString>,
}

/// Mac OS X uses first 8 bits as major number
fn major(dev: dev_t) -> dev_t {
    (dev >> 24) & 0xff
}

/// Mac OS X uses last 24 bits as minor number
fn minor(dev: dev_t) -> dev_t {
    dev & 0xffffff
}

fn main() -> Result<()> {
    let Opt { pathnames } = Opt::from_args();
    for path in pathnames {
        unsafe {
            let mut buf: stat = mem::zeroed();
            skip_fail!(stat(path.as_ptr(), &mut buf).non_negative());

            print!(
                "{}: dev = {}/{}",
                path.into_string()?,
                major(buf.st_dev),
                minor(buf.st_dev)
            );

            if buf.st_mode & S_IFMT == S_IFCHR {
                print!(
                    " character rdev = {}/{}",
                    major(buf.st_rdev),
                    minor(buf.st_rdev)
                );
            }
            if buf.st_mode & S_IFMT == S_IFBLK {
                print!(
                    " block rdev = {}/{}",
                    major(buf.st_rdev),
                    minor(buf.st_rdev)
                );
            }

            println!();
        }
    }

    Ok(())
}
