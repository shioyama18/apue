use anyhow::Result;
use apue::result::{NumericResult, PointerResult};
use libc::{
    closedir, lstat, opendir, readdir, stat, S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT,
    S_IFREG, S_IFSOCK,
};
use std::ffi::{CStr, CString};
use std::{cmp, fmt, mem};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = CString::new))]
    pathname: CString,
}

#[derive(Debug, Default)]
struct Counter {
    nreg: i32,
    ndir: i32,
    nblk: i32,
    nchr: i32,
    nfifo: i32,
    nslink: i32,
    nsock: i32,
}

impl Counter {
    fn ftw(&mut self, pathname: &CStr) -> Result<()> {
        unsafe {
            let mut statbuf: stat = mem::zeroed();
            lstat(pathname.as_ptr(), &mut statbuf).non_negative()?;

            match statbuf.st_mode & S_IFMT {
                S_IFDIR => {
                    self.ndir += 1;
                    let dp = opendir(pathname.as_ptr()).not_null()?;
                    while let Ok(dirp) = readdir(dp).not_null() {
                        let d_name = CStr::from_ptr((*dirp).d_name.as_ptr()).to_str()?;
                        if d_name != "." && d_name != ".." {
                            let new_path =
                                CString::new(format!("{}/{}", pathname.to_str()?, d_name))?;
                            self.ftw(&new_path)?;
                        }
                    }
                    closedir(dp).non_negative()?;
                }
                S_IFREG => self.nreg += 1,
                S_IFBLK => self.nblk += 1,
                S_IFCHR => self.nchr += 1,
                S_IFIFO => self.nfifo += 1,
                S_IFLNK => self.nslink += 1,
                S_IFSOCK => self.nsock += 1,
                _ => (),
            }
        }

        Ok(())
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ntot = cmp::max(
            1,
            self.nreg + self.ndir + self.nblk + self.nchr + self.nfifo + self.nslink + self.nsock,
        ) as f64;

        write!(
            f,
            r"regular files  = {:>7}, {:5.2}%
directories    = {:>7}, {:5.2}%
block special  = {:>7}, {:5.2}%
char special   = {:>7}, {:5.2}%
FIFOs          = {:>7}, {:5.2}%
symbolic links = {:>7}, {:5.2}%
sockets        = {:>7}, {:5.2}%",
            self.nreg,
            self.nreg as f64 * 100.0 / ntot,
            self.ndir,
            self.ndir as f64 * 100.0 / ntot,
            self.nblk,
            self.nblk as f64 * 100.0 / ntot,
            self.nchr,
            self.nchr as f64 * 100.0 / ntot,
            self.nfifo,
            self.nfifo as f64 * 100.0 / ntot,
            self.nslink,
            self.nslink as f64 * 100.0 / ntot,
            self.nsock,
            self.nsock as f64 * 100.0 / ntot,
        )
    }
}

fn main() -> Result<()> {
    let Opt { pathname } = Opt::from_args();
    let mut counter: Counter = Default::default();

    counter.ftw(&pathname)?;
    println!("{}", counter);

    Ok(())
}
