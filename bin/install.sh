#!/bin/sh
set -e

# Use bash for RISC Zero install as it requires bash features
echo "curling risczero"
curl -L https://risczero.com/install | bash

# Directly export RISC0 bin directory to PATH for current session
echo "Updating PATH for current session..."
export PATH="$HOME/.risc0/bin:$PATH"

# Check if rzup is available in PATH after direct export
if ! command -v rzup >/dev/null 2>&1; then
    echo "Note: You may need to restart your shell or source your shell config file"
    echo "to use rzup normally in the future."
    echo "Using full path to rzup for installation..."
    "$HOME/.risc0/bin/rzup" install cargo-risczero 2.3.1
else
    echo "rzup found in PATH, proceeding with installation..."
    rzup install cargo-risczero 2.3.1
fi

# check os linux or mac
OS=$(uname -s)
echo "Detected operating system: $OS"
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
        RUSTFLAGS="-C target-cpu=native" cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features linux --locked --force
    else
        echo "installing without cuda support, proving will be slower"
        # Install without any specific features for non-CUDA systems
        RUSTFLAGS="-C target-cpu=native" cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --locked --force
    fi
    ;;
Darwin)
    # Install flatc if not present
    if ! command -v flatc >/dev/null 2>&1; then
        echo "Installing flatc 24.3.25..."
        brew update
        brew install cmake
        cmake --version   # Should show version 3.28.3 or later
        brew install make gcc
        make --version   # Should show version 3.81 or later
        # Create a temporary directory for building
        cd /tmp
        # Clone the FlatBuffers repository
        git clone https://github.com/google/flatbuffers.git
        # Enter the repository directory
        cd flatbuffers
        # Checkout the specific version
        git checkout v24.3.25
        # Build FlatBuffers
        cmake -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release
        make
        # Move flatc compiler to /usr/local/bin
        sudo mv flatc /usr/local/bin/
        # Clean up
        cd ..
        rm -rf flatbuffers
        # Verify the installation
        flatc --version   # Should show version 24.3.25
        echo "flatc installation complete"
    fi
    cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features mac --locked --force
    ;;
*)
    echo "Unsupported operating system: $OS"
    exit 1
    ;;
esac
