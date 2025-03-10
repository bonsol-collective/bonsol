#!/bin/sh
set -e

# Use bash for RISC Zero install as it requires bash features
curl -L https://risczero.com/install | bash
rzup install cargo-risczero 1.2.1

# check os linux or mac
OS=$(uname -s)
case "$OS" in
Linux)
    # check if nvidia-smi exists and nvcc is available
    if command -v nvidia-smi >/dev/null 2>&1 && command -v nvcc >/dev/null 2>&1; then
        echo "installing with cuda support"
        # Install with specific Solana version to ensure compatibility
        RUSTFLAGS="-C target-cpu=native" cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features linux --no-default-features --features gpu
    else
        echo "installing without cuda support, proving will be slower"
        # Install with specific Solana version to ensure compatibility
        RUSTFLAGS="-C target-cpu=native" cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --no-default-features --features cpu
    fi
    ;;
Darwin)
    cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features mac
    ;;
*)
    echo "Unsupported operating system: $OS"
    exit 1
    ;;
esac
