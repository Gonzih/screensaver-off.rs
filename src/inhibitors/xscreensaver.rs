mod sh;

use self::sh::{exec, is_executable};
use log::{info, warn};

use sysinfo;

const PATH: &str = "xscreensaver";

fn is_applicable() -> bool {
    let sys = sysinfo::System::new();
    let procs = sys.get_process_list();
    let xscreensaver_running = procs
        .iter()
        .any(|(_, proc_)| proc_.name.starts_with("xscreensaver"));

    is_executable(PATH) && xscreensaver_running
}

pub fn disable() {
    if is_applicable() {
        info!("Disabling xscreensaver");
        exec(PATH, &["-deactivate"]);
    }
}

pub fn enable() {}
