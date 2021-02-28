use apue::c_to_string;
use apue::result::PointerResult;
use libc::{endpwent, getpwent, passwd, setpwent};
use std::ffi::{CStr, CString};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = CString::new))]
    username: CString,
}

unsafe fn getpwnam(name: &CStr) -> Option<passwd> {
    setpwent();

    while let Ok(ptr) = getpwent().not_null() {
        if name == CStr::from_ptr((*ptr).pw_name) {
            // Entry found
            endpwent();
            return Some(*ptr);
        }
    }

    // Entry not found
    endpwent();
    None
}

fn main() {
    let Opt { username } = Opt::from_args();
    unsafe {
        if let Some(passwd) = getpwnam(&username) {
            println!(
                "{}:{}:{}:{}:{}:{}:{}:{}:{}",
                c_to_string!(passwd.pw_name),
                c_to_string!(passwd.pw_passwd),
                passwd.pw_uid,
                passwd.pw_gid,
                passwd.pw_change,
                c_to_string!(passwd.pw_class),
                c_to_string!(passwd.pw_gecos),
                c_to_string!(passwd.pw_dir),
                c_to_string!(passwd.pw_shell),
            );
        }
    }
}
