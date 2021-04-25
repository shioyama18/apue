use anyhow::Result;
use apue::constants::FILE_MODE;
use apue::result::NumericResult;
use libc::{c_int, fstat, ftruncate, open, O_CREAT, O_RDONLY, O_RDWR, O_TRUNC};
use libc::{memcpy, mmap, munmap, MAP_FAILED, MAP_SHARED, PROT_READ, PROT_WRITE};
use std::ffi::CString;
use std::{cmp, mem};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = CString::new))]
    fromfile: CString,
    #[structopt(parse(try_from_str = CString::new))]
    tofile: CString,
}

const COPYINCR: usize = 1024 * 1024 * 1024; // 1GB

fn main() -> Result<()> {
    let Opt { fromfile, tofile } = Opt::from_args();

    unsafe {
        let fdin = open(fromfile.as_ptr(), O_RDONLY).non_negative()?;
        let fdout = open(
            tofile.as_ptr(),
            O_RDWR | O_CREAT | O_TRUNC,
            FILE_MODE as c_int,
        )
        .non_negative()?;

        let mut sbuf = mem::zeroed();
        fstat(fdin, &mut sbuf).non_negative()?;
        ftruncate(fdout, sbuf.st_size).non_negative()?; // set output filesize

        let mut fsz = 0;
        while fsz < sbuf.st_size {
            let copysz = cmp::min((sbuf.st_size - fsz) as usize, COPYINCR);

            let src = mmap(0 as _, copysz, PROT_READ, MAP_SHARED, fdin, fsz);
            assert_ne!(src, MAP_FAILED, "mmap error for input");
            let dst = mmap(
                0 as _,
                copysz,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                fdout,
                fsz,
            );
            assert_ne!(dst, MAP_FAILED, "mmap error for output");

            memcpy(dst, src, copysz);
            munmap(src, copysz);
            munmap(dst, copysz);
            fsz += copysz as i64;
        }
    }
    Ok(())
}
