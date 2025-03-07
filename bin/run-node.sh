#!/usr/bin/env bash

# Bonsol Node Runner Script
# ========================
#
# This script runs a Bonsol node with configurable logging and execution options.
# Note: This script only runs the node service. Deployment should be done separately from the client side.
#
# Usage:
#   ./run-node.sh [-F cuda] [-L] [-d]
#
# Options:
#   -F cuda        Enable CUDA support for GPU acceleration
#   -L            Use local build instead of installed bonsol
#   -d            Enable debug logging for all relevant modules
#
# Log Levels (from least to most verbose):
#   error         Show errors only
#   warn          Show warnings and errors
#   info          Show general information (default)
#   debug         Show detailed debugging information
#   trace         Show all possible logging information
#
# Common Usage Examples:
#   1. Basic run with debug logging:
#      ./run-node.sh -d
#
#   2. Local build with CUDA:
#      ./run-node.sh -L -F cuda
#
# Key Debug Targets:
#   risc0_runner       Image downloads, proofs, and claims
#   transaction_sender Transaction processing and status
#   input_resolver     Input processing and validation
#   reqwest            HTTP client logs (useful for S3 download issues)
#   hyper              Low-level HTTP details
#
# Recommended Debug Combinations:
#   Deployment issues:    -debug
#   Proof generation:     -debug
#   Full system debug:    -debug
#   S3 download issues:   -debug
#
# Common Issues:
#   1. S3 403 Forbidden: Use -debug to see full request details
#   2. Image download fails: Use -debug to see download URLs and attempts
#   3. Proof generation issues: Use -debug for detailed proving logs
#
# Note: On macOS, CUDA is not supported, and Arm CPUs cannot run the stark to snark prover.

set -e

NKP=node_keypair.json
USE_CUDA=false
USE_LOCAL_BUILD=false
DEBUG_MODE=false

while getopts "F:Ld" opt; do
  case $opt in
    F)
      if [ "$OPTARG" = "cuda" ]; then
        USE_CUDA=true
      else
        echo "Error: Unknown feature flag: $OPTARG"
        exit 1
      fi
      ;;
    L)
      USE_LOCAL_BUILD=true
      ;;
    d)
      DEBUG_MODE=true
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      echo "Usage: $0 [-F cuda] [-L] [-d]" >&2
      echo "  -F cuda: Enable CUDA support" >&2
      echo "  -L: Use local build" >&2
      echo "  -d: Enable debug mode" >&2
      exit 1
      ;;
  esac
done

# Create keypair if it doesn't exist
if [ -f $NKP ]; then
    echo "Bonsol node keypair exists"
else
    echo "Creating new node keypair..."
    solana-keygen new --outfile $NKP
fi

# Request SOL for transaction fees if needed
echo "Requesting SOL airdrop for node operations..."
solana -u http://localhost:8899 airdrop 1 --keypair node_keypair.json
solana -u http://localhost:8899 airdrop 1

# Set stack size limit
ulimit -s unlimited
# Increase memory limits
ulimit -v unlimited  # Virtual memory
ulimit -m unlimited  # Max memory size

# Enable and configure core dumps
ulimit -c unlimited  # Enable core dumps
echo "kernel.core_pattern=/tmp/core-%e-%p-%t" | sudo tee /proc/sys/kernel/core_pattern
mkdir -p /tmp/cores
chmod 777 /tmp/cores

# Display system limits and core dump configuration
echo ""
echo "============================================"
echo "🔧 System Configuration:"
echo "Stack size limit (kb): $(ulimit -s)"
echo "Virtual memory limit (kb): $(ulimit -v)"
echo "Max memory size (kb): $(ulimit -m)"
echo "Core file size limit (blocks): $(ulimit -c)"
echo "Core pattern: $(cat /proc/sys/kernel/core_pattern)"
echo "============================================"
echo ""

# Set logging configuration
if [ "$DEBUG_MODE" = true ]; then
    # Set specific module log levels for better debugging
    export RUST_LOG="bonsol_node=debug,bonsol_node::risc0_runner=trace,bonsol_prover=debug,bonsol_prover::input_resolver=trace,stark_to_snark=trace"
    echo ""
    echo "============================================"
    echo "🔍 DEBUG MODE ENABLED"
    echo "Log levels:"
    echo "  - bonsol_node = debug"
    echo "  - bonsol_node::risc0_runner = trace"
    echo "  - bonsol_prover = debug"
    echo "  - bonsol_prover::input_resolver = trace"
    echo "  - stark_to_snark = trace"
    echo "  - reqwest = trace"
    echo "  - hyper = debug"
    echo "  - others = debug"
    echo "============================================"
    echo ""
    
    echo "Debug: Build Configuration:"
    echo "  Current directory: $(pwd)"
    echo "  Using local build: $USE_LOCAL_BUILD"
    echo "  CUDA enabled: $USE_CUDA"
    echo "  Platform: $OSTYPE"
    echo "  Log level: RUST_LOG=$RUST_LOG"
    echo ""
    
    if [ "$USE_LOCAL_BUILD" = true ]; then
        # Point to debug build path
        BINARY_PATH="target/debug/bonsol-node"
        if [ -f "$BINARY_PATH" ]; then
            echo "Debug: Bonsol node binary details:"
            ls -l "$BINARY_PATH"
            echo "Debug: Binary last modified:"
            stat "$BINARY_PATH"
        else
            echo "Debug: No debug build found at $BINARY_PATH"
            echo "Debug: Building in debug mode..."
        fi
    fi
    echo "============================================"
    echo ""
else
    export RUST_LOG="info"
fi

# Function to run the node with environment variables
run_node() {
    # Run the actual command
    RUST_LOG="$RUST_LOG" "$@"
}

# Run the node based on platform and configuration
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "Running with RUST_LOG=$RUST_LOG"
    if [ "$USE_CUDA" = true ]; then
        if [ "$DEBUG_MODE" = true ]; then
            # Force rebuild in debug mode
            cargo clean -p bonsol-node
            run_node cargo run -p bonsol-node --features cuda -- -f ./Node.toml
        else
            run_node cargo run --release -p bonsol-node --features cuda -- -f ./Node.toml
        fi
    else
        if [ "$DEBUG_MODE" = true ]; then
            # Force rebuild in debug mode
            cargo clean -p bonsol-node
            run_node cargo run -p bonsol-node -- -f ./Node.toml
        else
            run_node cargo run --release -p bonsol-node -- -f ./Node.toml
        fi
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Running with RUST_LOG=$RUST_LOG"
    if [ "$USE_CUDA" = true ]; then
        echo "Error: CUDA is not supported on macOS"
        exit 1
    else
        echo "NOTE: MAC Arm CPUs will not be able to run the stark to snark prover, this is a known issue"
        run_node cargo run --release -p bonsol-node --features metal -- -f ./Node.toml
    fi
else
    echo "Unsupported operating system"
    exit 1
fi
