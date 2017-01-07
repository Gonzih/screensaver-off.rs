#[macro_use]
extern crate log;
extern crate env_logger;
extern crate regex;
extern crate gtk;
extern crate sysinfo;

use regex::Regex;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::env::home_dir;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::thread::sleep;
use std::time::Duration;
use gtk::prelude::*;
use gtk::StatusIcon;
use std::sync::{Arc, Mutex};
use std::thread;

const INACTIVE_ICON: &'static str = "caffeine-cup-empty";
const INACTIVE_TOOLTIP: &'static str = "Disable screensaver";
const ACTIVE_ICON: &'static str = "caffeine-cup-full";
const ACTIVE_TOOLTIP: &'static str = "Enable screensaver";

#[derive(Debug)]
struct AppState {
    manually_triggered: bool,
    automatically_triggered: bool,
}

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

fn check_and_disable_screensaver(state: &Arc<Mutex<AppState>>) {
    let mut state = state.lock().unwrap();

    if state.manually_triggered {
        info!("Disabling screensaver because forced by global state which is {:?}",
              *state);
        disable_xscreensaver();
    } else {
        let sys = sysinfo::System::new();
        let procs = sys.get_process_list();
        let regs = read_config();
        state.automatically_triggered = false;

        'outer: for (pid, proc_) in procs {
            for reg in &regs {
                let pname = proc_.name.as_str();
                if reg.is_match(pname) {
                    info!("Found matching process {} {}", pid, pname);
                    disable_xscreensaver();
                    state.automatically_triggered = true;
                    break 'outer;
                }
            }
        }
    }
}

fn start_monitoring_loop(state: Arc<Mutex<AppState>>) {
    loop {
        check_and_disable_screensaver(&state);
        sleep(Duration::from_secs(60));
    }
}

fn configure_icon(state: Arc<Mutex<AppState>>, icon: &StatusIcon) {
    icon.set_tooltip_text(INACTIVE_TOOLTIP);
    icon.set_visible(true);

    icon.connect_activate(move |i| {
        let mut state = state.lock().unwrap();
        state.manually_triggered = !state.manually_triggered;
        adjust_icon_pic(&state, &i);
    });

}

fn adjust_icon_pic(state: &AppState, icon: &StatusIcon) {
    if state.manually_triggered || state.automatically_triggered {
        icon.set_tooltip_text(ACTIVE_TOOLTIP);
        icon.set_from_icon_name(ACTIVE_ICON);
    } else {
        icon.set_tooltip_text(INACTIVE_TOOLTIP);
        icon.set_from_icon_name(INACTIVE_ICON);
    }
}

fn main() {
    env_logger::init().unwrap();

    if gtk::init().is_err() {
        panic!("Failed to initialize GTK!");
    }

    let shared_state = Arc::new(Mutex::new(AppState {
        manually_triggered: false,
        automatically_triggered: false,
    }));

    let icon = StatusIcon::new_from_icon_name(INACTIVE_ICON);

    let state1 = shared_state.clone();
    configure_icon(state1, &icon);

    let state2 = shared_state.clone();
    thread::spawn(move || {
        start_monitoring_loop(state2);
    });

    let state3 = shared_state.clone();
    gtk::timeout_add_seconds(1, move || {
        let state = state3.lock().unwrap();
        adjust_icon_pic(&state, &icon);
        Continue(true)
    });

    gtk::main();
}
