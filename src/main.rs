extern crate log;
extern crate env_logger;
extern crate regex;
extern crate gtk;
extern crate sysinfo;
extern crate dirs;

use regex::Regex;
use dirs::home_dir;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::thread::sleep;
use std::time::Duration;
use gtk::prelude::*;
use gtk::StatusIcon;
use std::sync::{Arc, Mutex};
use std::thread;
use log::{info};
use sysinfo::{System, SystemExt};

mod inhibitors;

use inhibitors::{disable_all, enable_all};

const INACTIVE_ICON: &'static str = "caffeine-cup-empty";
const INACTIVE_TOOLTIP: &'static str = "Disable screensaver";
const ACTIVE_ICON: &'static str = "caffeine-cup-full";
const ACTIVE_TOOLTIP: &'static str = "Enable screensaver";

#[derive(Debug)]
struct AppState {
    manually_triggered: bool,
    automatically_triggered: bool,
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

fn check_and_disable(state: &Arc<Mutex<AppState>>) {
    let mut state = state.lock().unwrap();

    if state.manually_triggered {
        info!("Disabling screensaver because forced by global state which is {:?}",
              *state);
        disable_all();
        return;
    } else {
        let sys = System::new();
        let procs = sys.get_process_list();
        let regs = read_config();

        let should_auto_disable = procs.iter().any(|(pid, proc_)| {
            regs.iter().any(|reg| {
                let pname = proc_.name.as_str();
                let is_match = reg.is_match(pname);
                if is_match {
                    info!("Found matching process {} {}", pid, pname);
                }

                is_match
            })
        });

        state.automatically_triggered = should_auto_disable;

        if should_auto_disable {
            disable_all();
        }
    }

    enable_all();
}

fn start_monitoring_loop(state: Arc<Mutex<AppState>>) {
    loop {
        check_and_disable(&state);
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
    env_logger::init();

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
    thread::spawn(move || { start_monitoring_loop(state2); });

    let state3 = shared_state.clone();
    gtk::timeout_add_seconds(1, move || {
        let state = state3.lock().unwrap();
        adjust_icon_pic(&state, &icon);
        Continue(true)
    });

    gtk::main();
}
