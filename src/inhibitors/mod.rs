mod xscreensaver;
mod xset;
mod sh;

pub fn disable_all() {
    xscreensaver::disable();
    xset::disable();
}

pub fn enable_all() {
    xscreensaver::disable();
    xset::disable();
}
