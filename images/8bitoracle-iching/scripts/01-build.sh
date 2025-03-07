#!/bin/bash

# Exit on error
set -e

# Parse command line arguments
USE_LOCAL=false
DEBUG=false
REBUILD_BONSOL=false
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --local) USE_LOCAL=true; shift ;;
        --debug) DEBUG=true; shift ;;
        --rebuild-bonsol) REBUILD_BONSOL=true; shift ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
done

# Source environment variables
ENV_FILE="$(dirname "$0")/../.env"
if [ -f "$ENV_FILE" ]; then
    echo "Loading environment variables from $ENV_FILE"
    set -a  # automatically export all variables
    source "$ENV_FILE"
    set +a
else
    echo "Warning: .env file not found at $ENV_FILE"
    exit 1
fi

# Enable debug logging if --debug flag is passed
if [ "$DEBUG" = true ]; then
    echo "Debug mode enabled"
    # Set logging for both Rust and bonsol components
    export RUST_LOG="debug,bonsol=debug,risc0_runner=debug"
    export RUST_BACKTRACE=1
    export CARGO_TERM_VERBOSE=true
fi

# Store original directory
ORIGINAL_DIR=$(pwd)

# Step 0: Rebuild bonsol if requested
if [ "$REBUILD_BONSOL" = true ]; then
    echo "Step 0: Rebuilding bonsol node software..."
    if [ "$DEBUG" = true ]; then
        echo "Debug: Building bonsol in directory: $ORIGINAL_DIR"
        echo "Debug: Running cargo build with verbose output"
        cargo build --verbose
    else
        cargo build
    fi
    echo "Bonsol rebuild complete"
    echo
fi

echo "Step 1: Building Rust program..."
if [ "$DEBUG" = true ]; then
    echo "Debug: Build Configuration:"
    echo "  Current directory: $(pwd)"
    echo "  Original directory: $ORIGINAL_DIR"
    echo "  Environment:"
    echo "    RUST_LOG=$RUST_LOG"
    echo "    RUST_BACKTRACE=$RUST_BACKTRACE"
    echo "    CARGO_TERM_VERBOSE=$CARGO_TERM_VERBOSE"
    echo "    BONSOL_HOME=$BONSOL_HOME"
fi

echo "Changing to I Ching program directory..."
cd "$(dirname "$0")/.."  # Change to iching program directory (up one level from scripts)
if [ "$DEBUG" = true ]; then
    echo "Debug: Changed to directory: $(pwd)"
    echo "Debug: Contents of current directory:"
    ls -la
    echo "Debug: Cargo.toml contents:"
    cat Cargo.toml
fi

echo "Running cargo build..."
if [ "$DEBUG" = true ]; then
    echo "Debug: Running cargo with verbose output"
    cargo build --verbose
else
    cargo build
fi

echo
echo "Step 2: Building ZK program..."
echo "Changing back to project root..."
cd "$ORIGINAL_DIR"  # Return to original directory for bonsol build
if [ "$DEBUG" = true ]; then
    echo "Debug: Changed back to directory: $(pwd)"
fi

# Determine which bonsol to use
if [ "$USE_LOCAL" = true ]; then
    if [ -f "${BONSOL_HOME}/target/debug/bonsol" ]; then
        BONSOL_CMD="${BONSOL_HOME}/target/debug/bonsol"
        echo "Using local bonsol build: $BONSOL_CMD"
        if [ "$DEBUG" = true ]; then
            echo "Debug: Bonsol binary details:"
            ls -l "$BONSOL_CMD"
            echo "Debug: Bonsol binary last modified:"
            stat "$BONSOL_CMD"
        fi
    else
        echo "Error: Local bonsol build not found at ${BONSOL_HOME}/target/debug/bonsol"
        echo "Please build bonsol locally first using 'cargo build'"
        exit 1
    fi
else
    BONSOL_CMD="bonsol"
    echo "Using installed bonsol from PATH"
    if [ "$DEBUG" = true ]; then
        echo "Debug: Bonsol path:"
        which bonsol
    fi
fi

if [ "$DEBUG" = true ]; then
    echo "Debug: ZK Program Configuration:"
    echo "  Program path: images/8bitoracle-iching"
    echo "  Bonsol command: $BONSOL_CMD"
    if [ -f "images/8bitoracle-iching/manifest.json" ]; then
        echo "Debug: Current manifest contents:"
        cat images/8bitoracle-iching/manifest.json
    fi
fi

echo "Running bonsol build..."
if [ "$DEBUG" = true ]; then
    echo "Debug: Running bonsol build command: $BONSOL_CMD build --zk-program-path images/8bitoracle-iching"
    "$BONSOL_CMD" build --zk-program-path images/8bitoracle-iching
    echo "Debug: Build command completed"
else
    "$BONSOL_CMD" build --zk-program-path images/8bitoracle-iching
fi

echo
echo "Build complete! You can now run 02-deploy.sh to deploy the program."
echo
if [ "$DEBUG" = true ]; then
    echo "Debug: Final Status:"
    echo "  Rust program built in: $(dirname "$0")/../target"
    echo "  ZK program built in: images/8bitoracle-iching"
    if [ -f "images/8bitoracle-iching/manifest.json" ]; then
        echo "  Updated manifest contents:"
        cat images/8bitoracle-iching/manifest.json
    fi
    echo "  Environment variables for next steps:"
    echo "    BONSOL_S3_ENDPOINT=$BONSOL_S3_ENDPOINT"
    echo "    BONSOL_S3_BUCKET=$BONSOL_S3_BUCKET"
    echo "    RUST_LOG=$RUST_LOG"
fi

echo "Note: You can use the following flags:"
echo "      --local           Use local bonsol build from target/debug/"
echo "      --debug           Enable detailed logging"
echo "      --rebuild-bonsol  Rebuild the bonsol node software" 