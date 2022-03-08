#!/bin/sh

cargo install --git https://github.com/burbokop/cargo-condep.git
cargo condep configure
cargo run
