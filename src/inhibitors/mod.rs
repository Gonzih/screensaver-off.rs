mod xscreensaver;
mod xset;
mod sh;

use anyhow::Result;

pub fn disable_all() -> Result<()> {
    xscreensaver::disable()?;
    xset::disable()?;

    Ok(())
}

pub fn enable_all() -> Result<()> {
    xscreensaver::enable()?;
    xset::enable()?;

    Ok(())
}
