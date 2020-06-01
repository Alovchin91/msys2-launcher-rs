use anyhow::Result;
use msys2_launcher::{Config, MSystem, Settings};

struct Mingw64;

impl MSystem for Mingw64 {
    fn get_config_branch<'a>(config: &'a Config) -> Result<&'a Settings<'a>> {
        config.mingw64()
    }

    fn get_msystem_string() -> &'static str {
        "MINGW64"
    }
}

fn main() -> Result<()> {
    msys2_launcher::launch::<Mingw64>()
}
