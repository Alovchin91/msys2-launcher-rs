mod config;
mod launcher;

pub use config::{Config, Settings};
pub use launcher::launch;

use anyhow::Result;

pub trait MSystem {
    fn get_config_branch(config: &Config) -> Result<&Settings>;
    fn get_msystem_string() -> &'static str;
}
