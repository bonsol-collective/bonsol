#!/bin/sh
set -e

# Use bash for RISC Zero install as it requires bash features
curl -L https://risczero.com/install | bash
rzup install cargo-risczero 1.2.1

# check os linux or mac
OS=$(uname -s)
case "$OS" in
Linux)
    # Install build dependencies if needed
    if ! command -v cmake >/dev/null 2>&1 || ! command -v git >/dev/null 2>&1; then
        echo "Installing build dependencies..."
        if command -v apt-get >/dev/null 2>&1; then
            sudo apt-get update
            sudo apt-get install -y cmake git build-essential
        elif command -v yum >/dev/null 2>&1; then
            sudo yum install -y cmake git gcc gcc-c++ make
        else
            echo "Could not find package manager. Please install cmake, git, and build tools manually."
            exit 1
        fi
    fi

    # Install flatc if not present
    if ! command -v flatc >/dev/null 2>&1; then
        echo "Installing flatc 24.3.25..."
        CURRENT_DIR=$(pwd)
        TEMP_DIR=$(mktemp -d)
        cd "$TEMP_DIR"
        git clone https://github.com/google/flatbuffers.git
        cd flatbuffers
        git checkout v24.3.25
        cmake -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release
        make
        sudo make install
        cd "$CURRENT_DIR"
        rm -rf "$TEMP_DIR"
        echo "flatc installation complete"
    fi

    # check if nvidia-smi exists and nvcc is available
    if command -v nvidia-smi >/dev/null 2>&1 && command -v nvcc >/dev/null 2>&1; then
        echo "installing with cuda support"
        RUSTFLAGS="-C target-cpu=native" cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features linux --locked
    else
        echo "installing without cuda support, proving will be slower"
        # Install without any specific features for non-CUDA systems
        RUSTFLAGS="-C target-cpu=native" cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --locked
    fi
    ;;
Darwin)
    # Install flatc if not present
    if ! command -v flatc >/dev/null 2>&1; then
        echo "Installing flatc 24.3.25..."
        brew install flatbuffers
        echo "flatc installation complete"
    fi
    cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features mac --locked
    ;;
*)
    echo "Unsupported operating system: $OS"
    exit 1
    ;;
esac
