# msys2-launcher-rs

This project is heavily inspired by the original [MSYS2 Launcher](https://github.com/msys2/msys2-launcher).

`msys2-launcher-rs` provides a set of executables for launching MSYS2 shell for different MSYSTEM's.

## Purpose

The original [MSYS2 Launcher](https://github.com/msys2/msys2-launcher) provides the ability to run MSYS2 shell within MinTTY terminal emulator.

Unfortunately, it doesn't play well with the alternative terminal emulators, like Alacritty, Hyper or Windows Terminal. The situation with the Windows Terminal at the time of writing is even more complicated, as it does not support specifying environment variables for profiles yet. This leaves the users of the Windows Terminal with a single option of using `msys2_shell.cmd` as a launcher.

The purpose of this project is to provide an alternative to the original MSYS2 Launcher which can be used from different terminal emulators, mainly the [Windows Terminal](https://github.com/microsoft/terminal).

Besides, I wanted to do a small project in Rust that would also be useful for myself. So here it goes :smile:

## Build & Install

To build, simply run `cargo build` (add `--release` for the release build) in the project directory.

To get the release build, you can also use artifacts from GitHub Actions. The Action additionally strips debug symbols from the resulting binaries which makes them considerably smaller.

Then copy `msys2.yml` and one or more of `msys2.exe`, `mingw32.exe` and `mingw64.exe` executables to the MSYS2 root directory.

## Configuration

The configuration is done using a single `msys2.yml` file located next to the launcher executables.

The configuration YAML contains 3 sections, one section per executable. If you don't use an executable, you can omit its section.

For example, if you only use `mingw64.exe`, your configuration file might look like this:

```yaml
mingw64:
  shell: /usr/bin/bash
```

`shell` is the required parameter. The path must be relative to the launcher directory (MSYS2 root).

## Contributing

This project is a work in progress. Contributions are welcome!

Before submitting a PR, please run `cargo fmt` on the code, and make sure that the project builds and runs.
