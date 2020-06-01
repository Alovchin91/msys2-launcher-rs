mod config;
mod launcher;

pub use config::{Config, Options};
pub use launcher::launch;

pub trait MSystem {
    fn get_config_branch<'a>(config: &'a Config) -> &'a Options<'a>;
    fn get_msystem_string() -> &'static str;
}
