# msys2-launcher-rs

This project is heavily inspired by the original [MSYS2 Launcher](https://github.com/msys2/msys2-launcher).

`msys2-launcher-rs` provides a set of executables for launching MSYS2 shell for different MSYSTEM's.

## Build & Install

To build, simply run `cargo build` (add `--release` for the release build) in the project directory.

To get the release build, you can also use artifacts from GitHub Actions. The Action additionally strips debug symbols from the resulting binaries which makes them considerably smaller.

Then copy `msys2.toml` and one or more of `msys2.exe`, `mingw32.exe` and `mingw64.exe` executables to the MSYS2 root directory.

## Configuration

The configuration is done using a single `msys2.toml` file located next to the launcher executables.

The configuration TOML contains 3 sections, one section per executable. If you don't use an executable, you can omit its section.

For example, if you only use `mingw64.exe`, your configuration might look like this:

```toml
[mingw64]
shell = "/usr/bin/bash"
```

`shell` is the required parameter. The path must be relative to the launcher directory (MSYS2 root).

## Contributing

This project is a work in progress. Contributions are welcome!

Before submitting a PR, please run `cargo fmt` on the code, and make sure that the project builds and runs.
