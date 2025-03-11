#!/usr/bin/env bash
set -e

curl -L https://risczero.com/install | bash
rzup install cargo-risczero 1.2.1


# check os linux or mac
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # check if nvidia-smi exists
    if ! command -v nvidia-smi &> /dev/null
    then
        echo "installing without cuda support, proving will be slower"
        cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol
    else
        echo "installing with cuda support"
        cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features linux
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features mac
fi
