extern crate sysinfo;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate regex;
extern crate gtk;

use regex::Regex;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::env::home_dir;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::thread::sleep;
use std::time::Duration;
use gtk::StatusIcon;
use std::sync::{Arc, Mutex};
use std::thread;

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
        info!("Reading configuration");
        let buf = BufReader::new(f.unwrap());

        buf.lines()
            .map(|line| {
                let line = line.unwrap();
                Regex::new(&line).unwrap()
            })
            .collect()
    }
}

fn check_and_disable_screensaver(state: &Arc<Mutex<bool>>) {
    let state = state.lock().unwrap();

    if *state {
        info!("Disabling screensaver because forced by global state which is {}", *state);
        disable_xscreensaver();
    } else {
        let sys = sysinfo::System::new();
        let procs = sys.get_process_list();
        let regs = read_config();

        'outer: for (pid, proc_) in procs {
            for reg in regs.clone() {
                let pname = proc_.name.as_str();
                if reg.is_match(pname) {
                    info!("Found matching process {} {}", pid, pname);
                    disable_xscreensaver();
                    break 'outer;
                }
            }
        }
    }
}

fn start_loop(state: Arc<Mutex<bool>>) {
    loop {
        check_and_disable_screensaver(&state);
        sleep(Duration::from_secs(6));
    }
}

fn main() {
    env_logger::init().unwrap();

    if gtk::init().is_err() {
        panic!("Failed to initialize GTK!");
    }

    let inactive_icon = "caffeine-cup-empty";
    let inactive_tooltip = "Disable screensaver";
    let active_icon = "caffeine-cup-full";
    let active_tooltip = "Enable screensaver";

    let global_state = Arc::new(Mutex::new(false));
    let click_shared_state = global_state.clone();
    let loop_shared_state = global_state.clone();

    let icon = StatusIcon::new_from_icon_name(inactive_icon);
    icon.set_tooltip_text(inactive_tooltip);
    icon.set_visible(true);
    icon.connect_activate(move |i| {
        let mut state = click_shared_state.lock().unwrap();
        *state = !*state;

        if *state {
            i.set_tooltip_text(active_tooltip);
            i.set_from_icon_name(active_icon);
        } else {
            i.set_tooltip_text(inactive_tooltip);
            i.set_from_icon_name(inactive_icon);
        }
    });

    thread::spawn(move || {
        start_loop(loop_shared_state);
    });

    gtk::main();
}
