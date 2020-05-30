use anyhow::{anyhow, Context, Result};
use std::process::Command;

fn main() -> Result<()> {
    let launcher_exe = std::env::current_exe().context("Could not determine executable path.")?;

    let launcher_dir = launcher_exe
        .parent()
        .context("Could not find root directory.")?;

    let msys2_shell_path = launcher_dir.join("msys2_shell.cmd");

    if !msys2_shell_path.exists() {
        return Err(anyhow!(
            "Could not find msys2_shell.cmd in {}",
            msys2_shell_path.to_string_lossy()
        ));
    }

    Command::new(msys2_shell_path)
        .args(&["-mingw64"])
        .spawn()
        .context("Could not start msys2 shell.")?;

    Ok(())
}
