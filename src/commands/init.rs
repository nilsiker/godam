use crate::config::{self, Config};

pub fn run() -> Result<(), config::ConfigError> {
    if Config::get().is_err() {
        Config::init()?;
        println!("Project initialized. Next, add assets using godam add <name>");
    } else {
        println!("Project already initialized. Try adding assets using godam install <name>");
    }
    Ok(())
}
