#!/bin/bash
set -euo pipefail

# Install the Rust stdlib for the current target
rustup target add $TARGET

# Download the Raspberry Pi cross-compilation toolchain if needed
if [ "$TARGET" = "arm-unknown-linux-gnueabihf" ]
then
  git clone --depth=1 https://github.com/raspberrypi/tools.git /tmp/tools
  export PATH=/tmp/tools/arm-bcm2708/arm-linux-gnueabihf/bin:$PATH
fi

# Compile the binary for the current target
cargo build --target=$TARGET --release

# Package up the release binary
tar -C target/$TARGET/release -czf homer-$TRAVIS_TAG-$TARGET.tar.gz homer
