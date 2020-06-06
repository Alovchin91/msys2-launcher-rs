use anyhow::Result;
use msys2_launcher::{Config, MSystem, Settings};

struct Mingw32;

impl MSystem for Mingw32 {
    fn get_config_branch(config: &Config) -> Result<&Settings> {
        config.mingw32()
    }

    fn get_msystem_string() -> &'static str {
        "MINGW32"
    }
}

fn main() -> Result<()> {
    msys2_launcher::launch::<Mingw32>()
}
