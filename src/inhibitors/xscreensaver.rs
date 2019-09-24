use super::sh::{exec, is_executable};
use log::{info};
use sysinfo::{System, ProcessExt, SystemExt};

const PATH: &str = "xscreensaver";

fn is_applicable() -> bool {
    let mut sys = sysinfo::System::new();
    sys.refresh_processes();
    let procs = sys.get_process_list();
    let xscreensaver_running = procs
        .iter()
        .any(|(_, proc_)| proc_.name().starts_with("xscreensaver"));

    is_executable(PATH) && xscreensaver_running
}

pub fn disable() {
    if is_applicable() {
        info!("Disabling xscreensaver");
        exec(PATH, &["-deactivate"]);
    }
}

pub fn enable() {}
