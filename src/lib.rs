#![cfg(windows)]

mod config;

pub use config::{Config, Settings};

use anyhow::{anyhow, Context, Result};
use std::process::Command;

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

pub fn launch<T: MSystem>() -> Result<()> {
    let launcher_exe = std::env::current_exe().context("Could not determine executable path.")?;

    let launcher_dir = launcher_exe
        .parent()
        .context("Could not find root directory.")?;

    let config: Config = {
        let config_file = launcher_dir.join("msys2.yml");

        let config_yaml =
            std::fs::File::open(config_file).context("Could not open msys2.yml config file.")?;

        serde_yaml::from_reader(config_yaml)
            .context("Could not read config from msys2.yml file.")?
    };

    let config_settings = T::get_config_branch(&config)?;

    let shell = config_settings
        .shell()
        .trim_start_matches::<&[_]>(&['\\', '/'])
        .replace('/', "\\");

    let shell_path = launcher_dir.join(shell).with_extension("exe");

    if !shell_path.exists() {
        return Err(anyhow!(
            "Could not find shell executable in {}",
            shell_path.display()
        ));
    }

    std::env::set_var("MSYSTEM", T::get_msystem_string());

    for (env_name, env_var) in config_settings.env() {
        std::env::set_var(env_name, env_var);
    }

    let mut shell_child = Command::new(shell_path).args(&["--login"]).spawn()?;

    free_console();
    let shell_status = shell_child.wait()?;

    std::process::exit(shell_status.code().unwrap_or(0))
}

fn free_console() -> bool {
    extern "system" {
        fn FreeConsole() -> std::os::raw::c_int;
    }

    unsafe { FreeConsole() != 0 }
}
