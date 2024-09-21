use anyhow::Result;

use crate::{cache, config::Config};

pub fn run() -> Result<()> {
    if Config::get().is_err() {
        Config::init()?;
        cache::init()?;
        println!("Project initialized. Next, add assets using godam add <name>");
    } else {
        println!("Project already initialized. Try adding assets using godam add <name>");
    }
    Ok(())
}
