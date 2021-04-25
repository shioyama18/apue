#![allow(non_upper_case_globals)]

use anyhow::Result;
use apue::constants::FILE_MODE;
use apue::result::NumericResult;
use libc::{
    aio_error, aio_fsync, aio_read, aio_return, aio_suspend, aio_write, aiocb, EINPROGRESS,
};
use libc::{
    c_char, c_int, c_void, fstat, isalpha, open, O_CREAT, O_RDONLY, O_RDWR, O_SYNC, O_TRUNC,
    SIGEV_NONE,
};
use once_cell::sync::Lazy;
use std::ffi::CString;
use std::{mem, ptr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = CString::new))]
    infile: CString,
    #[structopt(parse(try_from_str = CString::new))]
    outfile: CString,
}

const BSZ: usize = 4096;
const NBUF: usize = 8;

struct Buf {
    op: Rwop,
    last: bool,
    aiocb: aiocb,
    data: [c_char; BSZ],
}

#[derive(Debug, Clone, Copy)]
enum Rwop {
    Unused,
    ReadPending,
    WritePending,
}

unsafe fn translate(c: c_char) -> c_char {
    if isalpha(c as i32) != 0 {
        match c {
            c if c >= 'n' as c_char => return c - 13,
            c if c >= 'a' as c_char => return c + 13,
            c if c >= 'N' as c_char => return c - 13,
            c => return c + 13,
        }
    }

    c
}

static mut bufs: Lazy<[Buf; NBUF]> = Lazy::new(|| unsafe { mem::zeroed() });

fn main() -> Result<()> {
    let Opt { infile, outfile } = Opt::from_args();
    unsafe {
        for i in 0..NBUF {
            bufs[i] = Buf {
                op: Rwop::Unused,
                last: false,
                aiocb: mem::zeroed(),
                data: mem::zeroed(),
            };
            bufs[i].aiocb.aio_sigevent.sigev_notify = SIGEV_NONE;
            bufs[i].aiocb.aio_buf = bufs[i].data.as_mut_ptr() as *mut c_void;
        }

        let ifd = open(infile.as_ptr(), O_RDONLY).non_negative()?;
        let ofd = open(
            outfile.as_ptr(),
            O_RDWR | O_CREAT | O_TRUNC,
            FILE_MODE as c_int,
        )
        .non_negative()?;
        let mut sbuf = mem::zeroed();
        fstat(ifd, &mut sbuf).non_negative()?;

        let mut aiolist: [*const aiocb; NBUF] = [ptr::null(); NBUF];
        let mut numop = 0;
        let mut off = 0;
        loop {
            for i in 0..NBUF {
                match bufs[i].op {
                    Rwop::Unused => {
                        // Read from the input file if more data remains unread
                        if off < sbuf.st_size {
                            bufs[i].op = Rwop::ReadPending;
                            bufs[i].aiocb.aio_fildes = ifd;
                            bufs[i].aiocb.aio_offset = off;
                            off += BSZ as i64;
                            if off >= sbuf.st_size {
                                bufs[i].last = true;
                            }
                            bufs[i].aiocb.aio_nbytes = BSZ;
                            aio_read(&mut bufs[i].aiocb).non_negative()?;
                            aiolist[i] = &bufs[i].aiocb;
                            numop += 1;
                        }
                    }
                    Rwop::ReadPending => {
                        let err = aio_error(&bufs[i].aiocb);
                        if err == EINPROGRESS {
                            continue;
                        }
                        err.zeroed()?;

                        // Read is complete;
                        // Translate the buffer and write it.
                        let n = aio_return(&mut bufs[i].aiocb);
                        assert!(n >= 0, "aio_return failed");
                        let n = n as usize;
                        assert!(n == BSZ || bufs[i].last, "short read ({}/{})", n, BSZ);

                        for j in 0..n {
                            bufs[i].data[j] = translate(bufs[i].data[j]);
                        }
                        bufs[i].op = Rwop::WritePending;
                        bufs[i].aiocb.aio_fildes = ofd;
                        bufs[i].aiocb.aio_nbytes = n;
                        aio_write(&mut bufs[i].aiocb).non_negative()?;
                    }
                    Rwop::WritePending => {
                        let err = aio_error(&bufs[i].aiocb);
                        if err == EINPROGRESS {
                            continue;
                        }
                        err.zeroed()?;

                        // Write is complete;
                        // Mark the buffer as unused;
                        let n = aio_return(&mut bufs[i].aiocb);
                        assert!(n >= 0, "aio_return failed");
                        let n = n as usize;
                        assert!(n == bufs[i].aiocb.aio_nbytes, "short write ({}/{})", n, BSZ);
                        aiolist[i] = ptr::null();
                        bufs[i].op = Rwop::Unused;
                        numop -= 1;
                    }
                }
            }

            if numop == 0 {
                if off >= sbuf.st_size {
                    break;
                }
            } else {
                aio_suspend(aiolist.as_ptr(), NBUF as i32, ptr::null()).non_negative()?;
            }
        }

        bufs[0].aiocb.aio_fildes = ofd;
        aio_fsync(O_SYNC, &mut bufs[0].aiocb).non_negative()?;
    }

    Ok(())
}
