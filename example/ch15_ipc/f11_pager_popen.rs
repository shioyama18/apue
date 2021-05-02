use anyhow::Result;
use apue::c_str;
use apue::result::NumericResult;
use libc::{ferror, fgets, fopen, fputs, pclose, popen, EOF};
use std::ffi::CString;
use std::ptr;
use structopt::StructOpt;

const MAXLINE: usize = 1024;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = CString::new))]
    pathname: CString,
}

fn main() -> Result<()> {
    let Opt { pathname } = Opt::from_args();
    unsafe {
        let fpin = fopen(pathname.as_ptr(), c_str!("r"));
        assert_ne!(fpin, ptr::null_mut(), "can't open {:?}", pathname);
        let fpout = popen(c_str!("${PAGER:-more}"), c_str!("w"));
        assert_ne!(fpout, ptr::null_mut(), "popen error");

        let mut line = [0; MAXLINE];

        while fgets(line.as_mut_ptr(), MAXLINE as i32, fpin) != ptr::null_mut() {
            fputs(line.as_ptr(), fpout).not_equals(EOF)?;
        }

        ferror(fpin).zeroed()?;
        pclose(fpout).non_negative()?;
    }

    Ok(())
}
