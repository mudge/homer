set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    mkdir -p ~/.cargo
    cat > ~/.cargo/config <<EOF
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF

    cargo build --target $TARGET --release

    cp target/$TARGET/release/homer $stage/

    cd $stage
    tar czf $src/homer-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
