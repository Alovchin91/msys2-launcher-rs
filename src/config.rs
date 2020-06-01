use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config<'a> {
    #[serde(borrow)]
    mingw64: Option<Settings<'a>>,

    #[serde(borrow)]
    mingw32: Option<Settings<'a>>,

    #[serde(borrow)]
    msys2: Option<Settings<'a>>,
}

#[derive(Deserialize)]
pub struct Settings<'a> {
    shell: &'a str,
}

impl<'a> Config<'a> {
    pub fn mingw64(&self) -> Result<&Settings> {
        self.mingw64
            .as_ref()
            .ok_or_else(|| anyhow!("mingw64 config is missing."))
    }

    pub fn mingw32(&self) -> Result<&Settings> {
        self.mingw32
            .as_ref()
            .ok_or_else(|| anyhow!("mingw32 config is missing."))
    }

    pub fn msys2(&self) -> Result<&Settings> {
        self.msys2
            .as_ref()
            .ok_or_else(|| anyhow!("msys config is missing."))
    }
}

impl<'a> Settings<'a> {
    pub fn shell(&self) -> &str {
        self.shell
    }
}
