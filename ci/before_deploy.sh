#!/bin/bash
set -euo pipefail

# Install the Rust stdlib for ARMv6
rustup target install arm-unknown-linux-gnueabihf

# Download the Raspberry Pi cross-compilation toolchain
git clone --depth=1 https://github.com/raspberrypi/tools.git /tmp/tools

# Compile the binary for the Raspberry Pi
env PATH=/tmp/tools/arm-bcm2708/arm-linux-gnueabihf/bin:$PATH \
  CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
  cargo build --target=arm-unknown-linux-gnueabihf --release

# Package up the release binary
tar -C target/arm-unknown-linux-gnueabihf/release -czf homer-$TRAVIS_TAG-arm-unknown-linux-gnueabihf.tar.gz homer
