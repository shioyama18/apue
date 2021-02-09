#![allow(dead_code)]

use libc::{closedir, opendir, readdir};
use std::ffi::{CStr, CString};
use std::{env, fs, io, process};

/// list_files using libc
unsafe fn list_files_c(directory: &str) -> io::Result<()> {
    let c_dir = CString::new(directory)?;
    let dp = opendir(c_dir.as_ptr());

    // Return Err if directory is not found
    if dp.is_null() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Can't open: {}", directory),
        ));
    }

    loop {
        let entry = readdir(dp);

        if entry.is_null() {
            break;
        }

        let d_name = CStr::from_ptr((*entry).d_name.as_ptr());
        if let Ok(d_name) = d_name.to_str() {
            println!("{}", d_name);
        }
    }

    closedir(dp);

    Ok(())
}

/// list_files without using libc
fn list_files(directory: &str) -> io::Result<()> {
    let mut entries = fs::read_dir(directory)?
        .filter_map(|entry| entry.map(|e| e.file_name()).ok())
        .collect::<Vec<_>>();

    entries.sort();

    for entry in entries {
        if let Ok(entry) = entry.into_string() {
            println!("{}", entry);
        }
    }

    Ok(())
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("usage: ls directory_name");
        process::exit(-1);
    }

    unsafe {
        if let Err(e) = list_files_c(&args[1]) {
            eprintln!("Error: {:?}", e);
            process::exit(-1);
        }
    }
}
