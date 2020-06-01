use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config<'a> {
    #[serde(borrow)]
    mingw64: Option<Options<'a>>,

    #[serde(borrow)]
    mingw32: Option<Options<'a>>,

    #[serde(borrow)]
    msys2: Option<Options<'a>>,
}

#[derive(Deserialize)]
pub struct Options<'a> {
    shell: &'a str,
}

impl<'a> Config<'a> {
    pub fn mingw64(&self) -> Result<&Options> {
        self.mingw64
            .as_ref()
            .ok_or_else(|| anyhow!("mingw64 config is missing."))
    }

    pub fn mingw32(&self) -> Result<&Options> {
        self.mingw32
            .as_ref()
            .ok_or_else(|| anyhow!("mingw32 config is missing."))
    }

    pub fn msys2(&self) -> Result<&Options> {
        self.msys2
            .as_ref()
            .ok_or_else(|| anyhow!("msys config is missing."))
    }
}

impl<'a> Options<'a> {
    pub fn shell(&self) -> &str {
        self.shell
    }
}
