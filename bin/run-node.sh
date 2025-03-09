#!/usr/bin/env bash

# Bonsol Node Runner Script
# ========================
#
# This script runs a Bonsol node with configurable logging and execution options.
# Note: This script only runs the node service. Deployment should be done separately from the client side.
#
# Usage:
#   ./run-node.sh [-F cuda] [-L] [-d] [-D]
#
# Options:
#   -F cuda        Enable CUDA support for GPU acceleration
#   -L            Use local build instead of installed bonsol
#   -d            Enable debug logging for all relevant modules
#   -D            Enable RISC0 development mode (fast, non-secure proofs)
#
# Environment Variables Set:
#   RUST_LOG      Controls logging verbosity
#   RUST_BACKTRACE Controls backtrace detail level
#   RISC0_DEV_MODE Development mode for fast proof generation (when -D used)
#
# Common Usage Examples:
#   1. Basic run with debug logging:
#      ./run-node.sh -d
#
#   2. Local build with CUDA:
#      ./run-node.sh -L -F cuda
#
#   3. Development mode with debug logging:
#      ./run-node.sh -L -d -D
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
DEV_MODE=false

print_usage() {
    echo "Usage: $0 [-F cuda] [-L] [-d] [-D] [-h]"
    echo "  -F cuda: Enable CUDA support"
    echo "  -L: Use local build"
    echo "  -d: Enable debug mode"
    echo "  -D: Enable RISC0 development mode (fast, non-secure proofs)"
    echo "  -h: Show this help message"
    exit 1
}

while getopts "F:LdDh" opt; do
  case $opt in
    F)
      if [ "$OPTARG" = "cuda" ]; then
        USE_CUDA=true
      else
        echo "Error: Unknown feature flag: $OPTARG"
        print_usage
      fi
      ;;
    L)
      USE_LOCAL_BUILD=true
      ;;
    d)
      DEBUG_MODE=true
      ;;
    D)
      DEV_MODE=true
      ;;
    h)
      print_usage
      ;;
    \?)
      echo "Invalid option: -$OPTARG"
      print_usage
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
echo "Setting up core dump configuration..."

# Ensure /tmp/cores exists with correct permissions
sudo mkdir -p /tmp/cores
sudo chmod 777 /tmp/cores
sudo chown $USER:$USER /tmp/cores

# Configure core pattern (handle both native Linux and WSL)
if [ -f /proc/sys/kernel/core_pattern ]; then
    # Save current pattern
    OLD_PATTERN=$(cat /proc/sys/kernel/core_pattern)
    echo "Previous core pattern: $OLD_PATTERN"
    
    # Set new pattern
    echo "/tmp/cores/core-%e-%p-%t" | sudo tee /proc/sys/kernel/core_pattern
    
    # Verify the pattern was set correctly
    NEW_PATTERN=$(cat /proc/sys/kernel/core_pattern)
    echo "New core pattern: $NEW_PATTERN"
    
    # Additional WSL-specific configuration
    if grep -qi microsoft /proc/version; then
        echo "WSL detected - applying additional core dump settings..."
        # Enable core dumps in WSL
        sudo sysctl -w kernel.core_uses_pid=1
        # Ensure core pattern is absolute path in WSL
        if [[ "$NEW_PATTERN" != /* ]]; then
            echo "/tmp/cores/core-%e-%p-%t" | sudo tee /proc/sys/kernel/core_pattern
        fi
    fi
else
    echo "Warning: Could not configure core pattern - /proc/sys/kernel/core_pattern not found"
fi

# Display system limits and core dump configuration
echo ""
echo "============================================"
echo "🔧 System Configuration:"
echo "Stack size limit (kb): $(ulimit -s)"
echo "Virtual memory limit (kb): $(ulimit -v)"
echo "Max memory size (kb): $(ulimit -m)"
echo "Core file size limit (blocks): $(ulimit -c)"
echo "Core pattern: $(cat /proc/sys/kernel/core_pattern)"
echo "System type: $(uname -a)"
if grep -qi microsoft /proc/version; then
    echo "Running under WSL"
fi
echo "Core dump directory:"
ls -la /tmp/cores/
echo "============================================"
echo ""

# Set logging configuration
if [ "$DEBUG_MODE" = true ]; then
    # Enable full backtrace for debugging
    export RUST_BACKTRACE=1
    
    # Enhanced logging configuration with trace levels for RISC0 and proof verification
    export RUST_LOG="bonsol_node=debug,\
bonsol_node::risc0_runner=debug,\
bonsol_prover=debug,\
bonsol_prover::input_resolver=trace,\
stark_to_snark=trace,\
sol_prover::input_resolver=trace"

    echo ""
    echo "============================================"
    echo "🔍 DEBUG MODE ENABLED"
    echo "Log levels:"
    echo "  - bonsol_node = debug"
    echo "  - bonsol_node::risc0_runner = trace"
    echo "  - bonsol_prover = debug"
    echo "  - bonsol_prover::input_resolver = trace"
    echo "  - stark_to_snark = trace"
    echo "  - sol_prover::input_resolver = trace"
    echo "============================================"
    echo ""

    # Add RISC0 specific debug info
    if [ "$DEV_MODE" = true ]; then
        echo "RISC0 Development Mode: Enabled (FAST, NON-SECURE PROOFS)"
        export RISC0_DEV_MODE=1
    else
        echo "RISC0 Development Mode: Disabled (SECURE PROOFS)"
    fi
    
    # Check if RISC0_INFO is set for profiling
    if [ -n "$RISC0_INFO" ]; then
        echo "RISC0 Profiling: Enabled"
    fi

    echo "Debug: Build Configuration:"
    echo "  Current directory: $(pwd)"
    echo "  Using local build: $USE_LOCAL_BUILD"
    echo "  CUDA enabled: $USE_CUDA"
    echo "  Dev mode enabled: $DEV_MODE"
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
    # Default logging for non-debug mode
    export RUST_LOG="info,bonsol_node::risc0_runner=info,bonsol_prover=info"
    if [ "$DEV_MODE" = true ]; then
        echo "RISC0 Development Mode: Enabled (FAST, NON-SECURE PROOFS)"
        export RISC0_DEV_MODE=1
    fi
fi

# Function to run the node with environment variables
run_node() {
    # Ensure core dumps are enabled for this process and its children
    ulimit -S -c unlimited
    ulimit -H -c unlimited
    
    echo "Starting node process with core dumps enabled"
    echo "Current process: $$"
    echo "Core dump settings:"
    echo "  Core size limit: $(ulimit -c)"
    echo "  Core pattern: $(cat /proc/sys/kernel/core_pattern)"
    echo "  Process limits:"
    ulimit -a
    
    # Run the actual command with core dump settings and process tracking
    # Preserve the environment variables by using env
    env RUST_LOG="$RUST_LOG" RUST_BACKTRACE="$RUST_BACKTRACE" RISC0_DEV_MODE="$RISC0_DEV_MODE" bash -c '
        echo "Child process starting: $$"
        ulimit -c unlimited
        echo "Child core size limit: $(ulimit -c)"
        echo "Using log level: $RUST_LOG"
        echo "Backtrace level: $RUST_BACKTRACE"
        echo "RISC0 dev mode: $RISC0_DEV_MODE"
        '"$*"'
    '
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
