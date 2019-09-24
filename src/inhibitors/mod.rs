mod sh;
mod xscreensaver;
mod xset;

use self::sh::{exec, is_executable};
use log::{info, warn};


pub fn disable_all() {
    xscreensaver::disable();
    xset::disable();
}

pub fn enable_all() {
    xscreensaver::disable();
    xset::disable();
}
