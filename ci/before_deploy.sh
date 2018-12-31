set -ex

main() {
    local src=$(pwd) \
          stage=$(mktemp -d)

    rustup target install arm-unknown-linux-gnueabihf

    git clone --depth=1 https://github.com/raspberrypi/tools.git
    export PATH=$src/tools/arm-bcm2708/arm-linux-gnueabihf/bin:$PATH
    export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc

    cargo build --target=arm-unknown-linux-gnueabihf --release

    cp target/arm-unknown-linux-gnueabihf/release/homer $stage/

    cd $stage
    tar czf $src/homer-$TRAVIS_TAG-arm-unknown-linux-gnueabihf.tar.gz *
    cd $src

    rm -rf $stage
}

main
