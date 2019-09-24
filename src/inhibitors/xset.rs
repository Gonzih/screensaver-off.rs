mod sh;

use self::sh::{exec, is_executable};
use log::{info, warn};

const PATH: &str = "xset";

fn is_applicable() -> bool {
    is_executable(PATH)
}

pub fn disable() {
    if is_applicable() {
        info!("Disabling Xorg DPMS and Screensaver");
        exec(PATH, &["s", "off", "-dpms"]);
    }
}

pub fn enable() {
    if is_applicable() {
        info!("Enabling Xorg DPMS and Screensaver");
        exec(PATH, &["s", "on", "+dpms"]);
    }
}
