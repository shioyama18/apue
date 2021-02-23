use anyhow::Result;
use apue::c_void;
use apue::constants::FILE_MODE;
use apue::result::NumericResult;
use libc::{creat, lseek, open, read, write, O_RDONLY, SEEK_CUR};
use std::ffi::CString;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = CString::new))]
    source: CString,
    #[structopt(parse(try_from_str = CString::new))]
    target: CString,
}

const BUFFSIZE: usize = 1;

fn main() -> Result<()> {
    let Opt { source, target } = Opt::from_args();

    unsafe {
        let src_fd = open(source.as_ptr(), O_RDONLY).non_negative()?;
        let tgt_fd = creat(target.as_ptr(), FILE_MODE);
        let buf = [0; BUFFSIZE];

        while let Ok(n) = read(src_fd, c_void!(buf), BUFFSIZE).positive() {
            // Mac OS X writes '0' byte for hole
            if buf[0] == 0 {
                lseek(tgt_fd, 1, SEEK_CUR).non_negative()?;
            } else {
                assert_eq!(
                    write(tgt_fd, c_void!(buf), n as usize),
                    n,
                    "Failed writing to target file."
                );
            }
        }
    }

    Ok(())
}
