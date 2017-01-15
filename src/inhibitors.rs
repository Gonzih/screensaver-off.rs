use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

pub fn is_executable(path: &str) -> bool {
    let meta_maybe = fs::metadata(path);

    if !meta_maybe.is_ok() {
        return false;
    }

    let meta = meta_maybe.unwrap();
    let mode = meta.permissions().mode();
    let is_executable = mode & 0o111 != 0;

    meta.is_file() && is_executable
}

trait Inhibitor {
    fn disable(&self);
    fn enable(&self);
}

struct Xscreensaver<'a> {
    path: &'a str,
}

impl<'a> Xscreensaver<'a> {
    fn new() -> Xscreensaver<'a> {
        Xscreensaver { path: "/usr/bin/xscreensaver-command"}
    }
}

impl<'a> Inhibitor for Xscreensaver<'a> {
    fn disable(&self) {
        info!("Disabling xscreensaver");
        if is_executable(self.path) {
            let status = Command::new(self.path).arg("-deactivate").status();
            match status {
                Ok(v) => info!("Process {} exited with status {}", self.path, v),
                Err(err) => warn!("Process {} exited with error \"{}\"", self.path, err),
            }
        } else {
            warn!("{} is not executable!", self.path)
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
    fn disable(&self) {
        info!("Disabling Xorg DPMS");
        if is_executable(self.path) {
            let status = Command::new(self.path).arg("-dpms").status();
            match status {
                Ok(v) => info!("Process {} exited with status {}", self.path, v),
                Err(err) => warn!("Process {} exited with error \"{}\"", self.path, err),
            }
        } else {
            warn!("{} is not executable!", self.path)
        }
    }

    fn enable(&self) {
        info!("Enabling Xorg DPMS");
        if is_executable(self.path) {
            let status = Command::new(self.path).arg("+dpms").status();
            match status {
                Ok(v) => info!("Process {} exited with status {}", self.path, v),
                Err(err) => warn!("Process {} exited with error \"{}\"", self.path, err),
            }
        } else {
            warn!("{} is not executable!", self.path)
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
