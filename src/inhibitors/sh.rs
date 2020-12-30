use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, ExitStatus};
use std::ffi::OsStr;
use anyhow::Result;

pub fn is_executable(path: &str) -> bool {
    let meta_maybe = fs::metadata(path);

    if !meta_maybe.is_ok() {
        return false;
    }

    let meta = meta_maybe.unwrap();
    let mode = meta.permissions().mode();
    let is_executable = mode & 0o111 != 0;

    meta.is_file() && is_executable
}

pub fn exec<S: AsRef<OsStr>>(path: &str, args: &[S]) -> Result<ExitStatus> {
    let status = Command::new(path).args(args).status()?;
    Ok(status)
}
