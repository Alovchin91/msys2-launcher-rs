use anyhow::{anyhow, Result};
use itertools::Itertools;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    mingw64: Option<Settings>,
    mingw32: Option<Settings>,
    msys2: Option<Settings>,
}

#[derive(Deserialize)]
pub struct Settings {
    shell: String,
    env: Option<Vec<String>>,
}

impl Config {
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

impl Settings {
    pub fn shell(&self) -> &str {
        &self.shell
    }

    pub fn env(&self) -> Vec<(&str, &str)> {
        self.env
            .iter()
            .flatten()
            .filter_map(|v| v.splitn(2, '=').collect_tuple())
            .collect()
    }
}
