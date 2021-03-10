use anyhow::Result;
use apue::c_str;
use apue::result::NumericResult;
use libc::{c_char, execl, fork, waitpid};
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use std::{env, fs, ptr};

fn main() -> Result<()> {
    let mut temp_dir = env::temp_dir();
    temp_dir.set_file_name("testinterp");
    let path = temp_dir.to_string_lossy().to_string();
    let mut f = fs::File::create(temp_dir)?;
    f.set_permissions(fs::Permissions::from_mode(0o755))?;
    f.write(format!("#!{}/bin/echoall foo\n", env::var("CARGO_HOME")?).as_bytes())?;

    unsafe {
        let pid = fork().non_negative()?;
        match pid {
            0 => execl(
                c_str!(path),
                c_str!("testinterp"),
                c_str!("myarg1"),
                c_str!("MY ARG2"),
                0 as *const c_char,
            )
            .non_negative()?,
            _ => waitpid(pid, ptr::null_mut(), 0).non_negative()?,
        };
    }
    Ok(())
}
