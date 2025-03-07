#!/bin/bash

# Exit on error
set -e

# Parse command line arguments
USE_LOCAL=false
DEBUG=false
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --local) USE_LOCAL=true; shift ;;
        --debug) DEBUG=true; shift ;;
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

echo "----------------------------------------"
echo "Starting image ID extraction process..."

# Extract image ID from input.json
INPUT_PATH="images/8bitoracle-iching/input.json"
echo "Looking for input file at: $INPUT_PATH"

if [ ! -f "$INPUT_PATH" ]; then
    echo "Error: Input file not found at $INPUT_PATH"
    echo "Please run 03-generate-input.sh first to create the input file."
    exit 1
fi

echo "Found input file. Contents:"
echo "----------------------------------------"
cat "$INPUT_PATH" | jq '.'
echo "----------------------------------------"

# Extract image ID using jq
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed. Please install jq first."
    exit 1
fi

echo "Extracting imageId from input file..."
export BONSOL_IMAGE_ID=$(jq -r '.imageId' "$INPUT_PATH")
if [ -z "$BONSOL_IMAGE_ID" ] || [ "$BONSOL_IMAGE_ID" = "null" ]; then
    echo "Error: Could not extract imageId from input.json"
    exit 1
fi

echo "Successfully extracted image ID: $BONSOL_IMAGE_ID"
echo "----------------------------------------"

# Enable debug logging if --debug flag is passed
if [ "$DEBUG" = true ]; then
    echo "Debug mode enabled"
    echo "Setting up logging configuration..."
    # Focus logging on risc0_runner and input resolver
    export RUST_LOG="info,risc0_runner=debug,bonsol_prover::input_resolver=debug"
    export RUST_BACKTRACE=1
    echo "RUST_LOG set to: $RUST_LOG"
fi

# Set BONSOL_S3_ENDPOINT with base URL only (no bucket)
if [ -n "$S3_ENDPOINT" ]; then
    echo "Configuring S3 settings..."
    # Remove any existing protocol and trailing slash
    S3_ENDPOINT_CLEAN=${S3_ENDPOINT#https://}
    S3_ENDPOINT_CLEAN=${S3_ENDPOINT_CLEAN#http://}
    S3_ENDPOINT_CLEAN=${S3_ENDPOINT_CLEAN%/}
    
    # Add https:// but NOT the bucket
    export BONSOL_S3_ENDPOINT="https://$S3_ENDPOINT_CLEAN"
    # Export bucket and path format
    export BONSOL_S3_BUCKET="${BUCKET:-8bitoracle}"
    export BONSOL_S3_PATH_FORMAT="iching-{image_id}"
    
    echo "S3 Configuration:"
    echo "  Base URL: $BONSOL_S3_ENDPOINT"
    echo "  Bucket: $BONSOL_S3_BUCKET"
    echo "  Path format: $BONSOL_S3_PATH_FORMAT"
    echo "  Image ID: $BONSOL_IMAGE_ID"
    
    # Show the final URL that will be constructed
    FINAL_URL="$BONSOL_S3_ENDPOINT/$BONSOL_S3_BUCKET/iching-$BONSOL_IMAGE_ID"
    echo "Final S3 URL will be: $FINAL_URL"
    echo "----------------------------------------"
fi

# Determine which bonsol to use
if [ "$USE_LOCAL" = true ]; then
    if [ -f "${BONSOL_HOME}/target/debug/bonsol" ]; then
        BONSOL_CMD="${BONSOL_HOME}/target/debug/bonsol"
        echo "Using local bonsol build: $BONSOL_CMD"
    else
        echo "Error: Local bonsol build not found at ${BONSOL_HOME}/target/debug/bonsol"
        echo "Please build bonsol locally first using 'cargo build'"
        exit 1
    fi
else
    BONSOL_CMD="bonsol"
    echo "Using installed bonsol from PATH"
fi

echo "----------------------------------------"
echo "Environment variables that will be used:"
echo "BONSOL_IMAGE_ID=$BONSOL_IMAGE_ID"
echo "BONSOL_S3_ENDPOINT=$BONSOL_S3_ENDPOINT"
echo "BONSOL_S3_BUCKET=$BONSOL_S3_BUCKET"
echo "BONSOL_S3_PATH_FORMAT=$BONSOL_S3_PATH_FORMAT"
echo "----------------------------------------"

echo "Executing I Ching program..."
"$BONSOL_CMD" execute -f "$INPUT_PATH" --wait

echo "Execution complete! Check the output above for your I Ching reading." 