use anyhow::Result;
use apue::c_str;
use apue::result::NumericResult;
use libc::STDIN_FILENO;
use libc::{
    c_char, close, dup2, execl, ferror, fgets, fopen, fork, pipe, strlen, strrchr, waitpid, write,
};
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
        let fp = fopen(pathname.as_ptr(), c_str!("r"));
        assert_ne!(fp, ptr::null_mut(), "can't open {:?}", pathname);
        let mut fd = [0; 2];
        pipe(fd.as_mut_ptr()).non_negative()?;
        let pid = fork().non_negative()?;
        match pid {
            0 => {
                // child
                close(fd[1]);
                if fd[0] != STDIN_FILENO {
                    dup2(fd[0], STDIN_FILENO).equals(STDIN_FILENO)?;
                    close(fd[0]);
                }

                let pager = "/usr/bin/more".as_ptr() as _;
                let mut argv = strrchr(pager, '/' as i32);
                if argv.is_null() {
                    argv = argv.offset(1);
                } else {
                    argv = pager as *mut _;
                }

                use std::ffi::CStr;
                println!("pager: {:?}", CStr::from_ptr(pager));
                println!("argv: {:?}", CStr::from_ptr(argv));
                execl(pager, argv, 0 as *const c_char).non_negative()?;
            }
            pid => {
                // parent
                close(fd[0]); // close read end
                let mut line = [0; MAXLINE];
                while fgets(line.as_mut_ptr(), MAXLINE as i32, fp) != ptr::null_mut() {
                    let n = strlen(line.as_ptr());
                    write(fd[1], line.as_ptr() as _, n).equals(n as isize)?;
                }
                ferror(fp).zeroed()?;
                close(fd[1]); // close write end of pipe for reader
                waitpid(pid, ptr::null_mut(), 0).non_negative()?;
            }
        }
    }

    Ok(())
}
