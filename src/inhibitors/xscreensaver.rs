use super::sh::{exec, is_executable};
use log::info;
use sysinfo::{ProcessExt, System, SystemExt};

const PATH: &str = "xscreensaver";

fn is_applicable() -> bool {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_processes();
    let procs = sys.get_processes();
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
