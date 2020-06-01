#!/usr/bin/env sh

export RUSTFLAGS="-C link-args=-Wl,--strip-debug"
cargo build --release
