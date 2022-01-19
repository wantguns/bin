#!/bin/bash

set -xe

# Setup cross compiling for arm64
setup_cross_compiling() {
    __ARCH=$1

    if [ "$__ARCH" == "arm64" ]; then
        sudo apt update
        sudo apt install -y gcc-aarch64-linux-gnu
        rustup target add aarch64-unknown-linux-gnu
    fi
}

# Building for the given architecture
build_for_arch() {
    __ARCH=$1
    __REF=$2

    # Statically compile
    case "$__ARCH" in \
        arm64) export __TARGET='aarch64-unknown-linux-gnu' ;;
        amd64) export __TARGET='x86_64-unknown-linux-gnu' ;;
    esac

    # Build and install
    cargo install --target $__TARGET --path .

    # Export them artifacts
    mkdir -p artifacts
    cp -av /home/runner/.cargo/bin/bin "artifacts/bin-$__REF-$__TARGET"
}