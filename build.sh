

if [[ "$1" == "--arm" ]]; then
    cargo build --target armv7-unknown-linux-gnueabi
else
    cargo build
fi