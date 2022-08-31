#!/bin/sh

rustup target add aarch64-apple-darwin
rustup target add aarch64-unknown-linux-gnu
#rustup target add aarch64-pc-windows-msvc

#cargo check
#cargo check --target aarch64-apple-darwin
#cargo check --target aarch64-unknown-linux-gnu
#cargo check --target aarch64-pc-windows-msvc

#cargo clippy --target aarch64-unknown-linux-gnu

cargo test --target aarch64-unknown-linux-gnu
