use sysinfo::{System, SystemExt};
use log::{info};

mod sh;

use self::sh::{exec, is_executable};

trait Inhibitor {
    fn disable(&self);
    fn enable(&self);
    fn is_applicable(&self) -> bool;
}

struct Xscreensaver<'a> {
    path: &'a str,
}

impl<'a> Xscreensaver<'a> {
    fn new() -> Xscreensaver<'a> {
        Xscreensaver { path: "/usr/bin/xscreensaver-command" }
    }
}

impl<'a> Inhibitor for Xscreensaver<'a> {
    fn is_applicable(&self) -> bool {
        let sys = System::new();
        let procs = sys.get_process_list();
        let xscreensaver_running =
            procs.iter().any(|(_, proc_)| proc_.name.starts_with("xscreensaver"));

        is_executable(self.path) && xscreensaver_running
    }

    fn disable(&self) {
        if self.is_applicable() {
            info!("Disabling xscreensaver");
            exec(self.path, &["-deactivate"]);
        }
    }

    fn enable(&self) {}
}

struct Xset<'a> {
    path: &'a str,
}

impl<'a> Xset<'a> {
    fn new() -> Xset<'a> {
        Xset { path: "/usr/bin/xset" }
    }
}

impl<'a> Inhibitor for Xset<'a> {
    fn is_applicable(&self) -> bool {
        is_executable(self.path)
    }

    fn disable(&self) {
        if self.is_applicable() {
            info!("Disabling Xorg DPMS and Screensaver");
            exec(self.path, &["s", "off", "-dpms"]);
        }
    }

    fn enable(&self) {
        if self.is_applicable() {
            info!("Enabling Xorg DPMS and Screensaver");
            exec(self.path, &["s", "on", "+dpms"]);
        }
    }
}

pub fn disable_all() {
    Xscreensaver::new().disable();
    Xset::new().disable();
}

pub fn enable_all() {
    Xscreensaver::new().enable();
    Xset::new().enable();
}
