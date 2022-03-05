#!/bin/sh

cargo install --git https://github.com/burbokop/conf_plugin.git
cargo generate config --target armv7-unknown-linux-gnueabi
cargo build --target armv7-unknown-linux-gnueabi
