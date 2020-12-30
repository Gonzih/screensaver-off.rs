use super::sh::{exec, is_executable};
use log::info;
use anyhow::Result;

const PATH: &str = "xset";

fn is_applicable() -> bool {
    is_executable(PATH)
}

pub fn disable() -> Result<()> {
    if is_applicable() {
        info!("Disabling Xorg DPMS and Screensaver");
        exec(PATH, &["s", "off", "-dpms"])?;
    }

    Ok(())
}

pub fn enable() -> Result<()> {
    if is_applicable() {
        info!("Enabling Xorg DPMS and Screensaver");
        exec(PATH, &["s", "on", "+dpms"])?;
    }

    Ok(())
}
