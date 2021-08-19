use anyhow::{anyhow, Result};
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
    config_getter! { mingw64 }
    config_getter! { mingw32 }
    config_getter! { msys2 }
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
