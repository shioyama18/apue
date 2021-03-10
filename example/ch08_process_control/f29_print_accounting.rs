use anyhow::Result;
use apue::bindings::{acct, ACORE, AFORK, ASU, AXSIG};
use apue::c_str;
use apue::result::PointerResult;
use libc::{c_void, fopen, fread};
use std::ffi::{CStr, CString};
use std::mem;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = CString::new))]
    filename: CString,
}

trait Flag {
    fn to_char(&self, flag: u32, c: char) -> char;
}

impl Flag for u8 {
    fn to_char(&self, flag: u32, c: char) -> char {
        if *self as u32 & flag == 0 {
            ' '
        } else {
            c
        }
    }
}

fn main() -> Result<()> {
    let Opt { filename } = Opt::from_args();
    unsafe {
        let mut acdata: acct = mem::zeroed();
        let fp = fopen(filename.as_ptr(), c_str!("r")).not_null()?;
        while fread(
            &mut acdata as *mut _ as *mut c_void,
            mem::size_of::<acct>(),
            1,
            fp,
        ) == 1
        {
            println!(
                "{:10} e = {:6}, chars = {:7}, {} {} {} {}",
                CStr::from_ptr(acdata.ac_comm.as_ptr()).to_string_lossy(),
                acdata.ac_etime,
                acdata.ac_io,
                acdata.ac_flag.to_char(ACORE, 'D'),
                acdata.ac_flag.to_char(AXSIG, 'X'),
                acdata.ac_flag.to_char(AFORK, 'F'),
                acdata.ac_flag.to_char(ASU, 'S'),
            );
        }
    }

    Ok(())
}
