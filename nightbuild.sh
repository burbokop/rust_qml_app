#!/bin/sh

cargo install --git https://github.com/burbokop/cargo-condep.git
cargo condep configure --target armv7-unknown-linux-gnueabi
cargo build --release
