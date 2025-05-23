#!/usr/bin/env bash

# Bonsol Node Runner Script
# ========================
#
# This script runs a Bonsol node with configurable logging and execution options.
# Note: This script only runs the node service. Deployment should be done separately from the client side.
#
# Usage:
#   ./run-node.sh [-F cuda] [-L] [-l log_level] [-t log_target]
#
# Options:
#   -F cuda        Enable CUDA support for GPU acceleration
#   -L            Use local build instead of installed bonsol
#   -l level      Set global log level (error|warn|info|debug|trace)
#   -t target     Set specific module log targets
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
#      ./run-node.sh -l debug
#
#   2. Debug specific modules:
#      ./run-node.sh -t "risc0_runner=debug"
#      ./run-node.sh -t "risc0_runner=debug,transaction_sender=debug"
#
#   3. Local build with CUDA:
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
#   Deployment issues:    -t "risc0_runner=debug,transaction_sender=debug"
#   Proof generation:     -t "risc0_runner=trace"
#   Full system debug:    -l debug
#   S3 download issues:   -t "risc0_runner=debug,reqwest=debug,hyper=debug"
#
# Common Issues:
#   1. S3 403 Forbidden: Use -t "risc0_runner=debug,reqwest=debug" to see full request details
#   2. Image download fails: Use -t "risc0_runner=debug" to see download URLs and attempts
#   3. Proof generation issues: Use -t "risc0_runner=trace" for detailed proving logs
#
# Note: On macOS, CUDA is not supported, and Arm CPUs cannot run the stark to snark prover.

set -e

NKP=node_keypair.json
USE_CUDA=false
USE_LOCAL_BUILD=false
LOG_LEVEL="info"
LOG_TARGET=""

while getopts "F:Ll:t:" opt; do
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
    l)
      LOG_LEVEL="$OPTARG"
      ;;
    t)
      LOG_TARGET="$OPTARG"
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      echo "Usage: $0 [-F cuda] [-L] [-l log_level] [-t log_target]" >&2
      echo "  -F cuda: Enable CUDA support" >&2
      echo "  -L: Use local build" >&2
      echo "  -l: Set log level (error|warn|info|debug|trace)" >&2
      echo "  -t: Set log target (e.g., risc0_runner=debug)" >&2
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

# Construct RUST_LOG environment variable
if [ -n "$LOG_TARGET" ]; then
    export RUST_LOG="$LOG_TARGET"
else
    export RUST_LOG="$LOG_LEVEL"
fi

echo "Using log configuration: RUST_LOG=$RUST_LOG"

# Run the node based on platform and configuration
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if [ "$USE_CUDA" = true ]; then
        cargo run --release -p bonsol-node --features cuda -- -f ./Node.toml
    else
        cargo run --release -p bonsol-node -- -f ./Node.toml
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    if [ "$USE_CUDA" = true ]; then
        echo "Error: CUDA is not supported on macOS"
        exit 1
    else
        echo "NOTE: MAC Arm CPUs will not be able to run the stark to snark prover, this is a known issue"
        cargo run --release -p bonsol-node --features metal -- -f ./Node.toml
    fi
else
    echo "Unsupported operating system"
    exit 1
fi
