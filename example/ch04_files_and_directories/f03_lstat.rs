use apue::c_str;
use apue::result::NumericResult;
use libc::{lstat, stat, S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK};
use std::{env, mem};

/// Taken from `man lstat`:
///
/// #define S_IFMT 0170000           /* type of file */
/// #define        S_IFIFO  0010000  /* named pipe (fifo) */
/// #define        S_IFCHR  0020000  /* character special */
/// #define        S_IFDIR  0040000  /* directory */
/// #define        S_IFBLK  0060000  /* block special */
/// #define        S_IFREG  0100000  /* regular */
/// #define        S_IFLNK  0120000  /* symbolic link */
/// #define        S_IFSOCK 0140000  /* socket */
/// #define        S_IFWHT  0160000  /* whiteout */
fn main() {
    let mut buf: stat = unsafe { mem::zeroed() };

    for arg in env::args().skip(1) {
        if let Err(e) = unsafe { lstat(c_str!(arg.as_str()), &mut buf).non_negative() } {
            eprintln!("lstat error: {}", e);
            continue;
        }

        let mode = match buf.st_mode & S_IFMT {
            S_IFREG => "regular",
            S_IFDIR => "directory",
            S_IFCHR => "character special",
            S_IFBLK => "block special",
            S_IFIFO => "fifo",
            S_IFLNK => "symbolic link",
            S_IFSOCK => "socket",
            _ => "** unknown mode **",
        };

        println!("{}: {}", arg, mode);
    }
}
