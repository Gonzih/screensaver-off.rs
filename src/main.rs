extern crate sysinfo;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate regex;

use regex::Regex;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::env::home_dir;
use std::fs::{File};
use std::io::{BufReader, BufRead};

fn is_executable(path: &str) -> bool {
    let meta_maybe = fs::metadata(path);

    if !meta_maybe.is_ok() {
        return false;
    }

    let meta = meta_maybe.unwrap();
    let mode = meta.permissions().mode();
    let is_executable = mode & 0o111 != 0;

    meta.is_file() && is_executable
}

fn disable_xscreensaver() {
    info!("Disabling xscreensaver");
    let path = "/usr/bin/xscreensaver-command";
    if is_executable(path) {
        let status = Command::new(path).arg("-deactivate").status();
        match status {
            Ok(v) => info!("Process {} exited with status {}", path, v),
            Err(err) => warn!("Process {} exited with error \"{}\"", path, err),
        }
    } else {
        warn!("{} is not executable!", path)
    }
}

fn read_config() -> Vec<Regex> {
    let path = format!("{}/.screensaver-off", home_dir().unwrap().display());
    let f = File::open(path);

    if !f.is_ok() {
        vec![]
    } else {
        let buf = BufReader::new(f.unwrap());

        buf.lines().map(|line| {
            let line = line.unwrap();
            Regex::new(&line).unwrap()
        }).collect()
    }
}

fn main() {
    env_logger::init().unwrap();
    let sys = sysinfo::System::new();
    let procs = sys.get_process_list();
    let regs = read_config();

    for (pid, proc_) in procs {
        for reg in regs.clone() {
            if reg.is_match(proc_.name.as_str()) {
                info!("Found matching process {} {}", pid, proc_.name);
                disable_xscreensaver();
                return;
            }
        }
    }
}
