mod config;
mod launcher;

pub use config::{Config, Settings};
pub use launcher::launch;

use anyhow::Result;

pub trait MSystem {
    fn get_config_branch(config: &Config) -> Result<&Settings>;
    fn get_msystem_string() -> &'static str;
}

#[macro_export]
macro_rules! define_msystem_bin {
    ($struct_name:ident, $config_branch:ident, $msystem_string:literal) => {
        use anyhow::Result;
        use msys2_launcher::{Config, MSystem, Settings};

        struct $struct_name;

        impl MSystem for $struct_name {
            fn get_config_branch(config: &Config) -> Result<&Settings> {
                config.$config_branch()
            }

            fn get_msystem_string() -> &'static str {
                $msystem_string
            }
        }

        fn main() -> Result<()> {
            msys2_launcher::launch::<$struct_name>()
        }
    };
}
