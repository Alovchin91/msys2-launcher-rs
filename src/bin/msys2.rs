use anyhow::Result;
use msys2_launcher::{Config, MSystem, Settings};

struct Msys2;

impl MSystem for Msys2 {
    fn get_config_branch<'a>(config: &'a Config) -> Result<&'a Settings<'a>> {
        config.msys2()
    }

    fn get_msystem_string() -> &'static str {
        "MSYS"
    }
}

fn main() -> Result<()> {
    msys2_launcher::launch::<Msys2>()
}
