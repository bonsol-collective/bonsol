#!/bin/sh
set -e

curl -L https://risczero.com/install | sh
rzup install cargo-risczero 1.2.1

# check os linux or mac
case "$OSTYPE" in
linux-gnu*)
    # check if nvidia-smi exists
    if ! command -v nvidia-smi >/dev/null 2>&1; then
        echo "installing without cuda support, proving will be slower"
        cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol
    else
        echo "installing with cuda support"
        cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features linux
    fi
    ;;
darwin*)
    cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features mac
    ;;
*)
    echo "Unsupported operating system: $OSTYPE"
    exit 1
    ;;
esac
