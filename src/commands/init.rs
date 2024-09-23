use console::style;

use crate::{
    config::{self, Config},
    BLUE, ORANGE,
};

pub fn run() -> Result<(), config::ConfigError> {
    if Config::get().is_err() {
        Config::init()?;
        println!(
            "{}",
            style("godam: Project initialized. Try searching for asset IDs using 'godam search <name>'")
                .color256(BLUE)
        );
    } else {
        println!(
            "{}",
            style("godam: Project already initialized. Try searching for asset IDs using 'godam search <name>'")
                .color256(ORANGE)
        )
    }
    Ok(())
}
