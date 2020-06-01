use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config<'a> {
    #[serde(borrow)]
    mingw64: Options<'a>,
}

#[derive(Deserialize)]
pub struct Options<'a> {
    shell: &'a str,
}

impl<'a> Config<'a> {
    pub fn mingw64(&self) -> &Options {
        &self.mingw64
    }
}

impl<'a> Options<'a> {
    pub fn shell(&self) -> &str {
        self.shell
    }
}
