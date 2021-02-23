use libc::{access, open, O_RDONLY, R_OK};
use std::ffi::CString;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = CString::new))]
    pathname: CString,
}

fn main() {
    let opt = Opt::from_args();

    unsafe {
        if access(opt.pathname.as_ptr(), R_OK) < 0 {
            eprintln!("access error for {:?}", opt.pathname);
        } else {
            println!("read access OK");
        }

        if open(opt.pathname.as_ptr(), O_RDONLY) < 0 {
            eprintln!("open error for {:?}", opt.pathname);
        } else {
            println!("open for reading OK");
        }
    }
}
