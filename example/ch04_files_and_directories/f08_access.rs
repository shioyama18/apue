use anyhow::Result;
use apue::result::NumericResult;
use libc::{access, open, O_RDONLY, R_OK};
use std::ffi::CString;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = CString::new))]
    pathname: CString,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    unsafe {
        access(opt.pathname.as_ptr(), R_OK).non_negative()?;
        println!("read access OK");

        open(opt.pathname.as_ptr(), O_RDONLY).non_negative()?;
        println!("open for reading OK");
    }

    Ok(())
}
