use crate::{Config, MSystem};
use anyhow::{anyhow, Context, Result};
use std::process::Command;

pub fn launch<T: MSystem>() -> Result<()> {
    let launcher_exe = std::env::current_exe().context("Could not determine executable path.")?;

    let launcher_dir = launcher_exe
        .parent()
        .context("Could not find root directory.")?;

    let config_file = launcher_dir.join("msys2.toml");
    let config_toml =
        std::fs::read_to_string(config_file).context("Could not find msys2.toml config file.")?;

    let config: Config = toml::from_str(&config_toml)?;

    let shell = T::get_config_branch(&config)?
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
