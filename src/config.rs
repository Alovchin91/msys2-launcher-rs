use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    msys2: Option<Settings>,
    mingw64: Option<Settings>,
    ucrt64: Option<Settings>,
    clang64: Option<Settings>,
    clangarm64: Option<Settings>,
    mingw32: Option<Settings>,
}

#[derive(Deserialize)]
pub struct Settings {
    shell: String,
    env: Option<Vec<String>>,
}

macro_rules! config_getter {
    ($msystem:ident) => {
        pub fn $msystem(&self) -> Result<&Settings> {
            self.$msystem
                .as_ref()
                .ok_or_else(|| anyhow!("{} config is missing.", stringify!($msystem)))
        }
    };
}

impl Config {
    config_getter! { msys2 }
    config_getter! { mingw64 }
    config_getter! { ucrt64 }
    config_getter! { clang64 }
    config_getter! { clangarm64 }
    config_getter! { mingw32 }
}

impl Settings {
    pub fn shell(&self) -> &str {
        &self.shell
    }

    pub fn env(&self) -> Vec<(&str, &str)> {
        self.env
            .iter()
            .flatten()
            .filter_map(|v| v.split_once('='))
            .collect()
    }
}
