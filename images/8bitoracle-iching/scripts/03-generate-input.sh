#!/bin/bash

# Exit on error
set -e

# Parse command line arguments
USE_LOCAL=false
ESTIMATE_CYCLES=false
IMAGE_ID=""

while [[ "$#" -gt 0 ]]; do
    case $1 in
        --local) USE_LOCAL=true; shift ;;
        --estimate-cycles) ESTIMATE_CYCLES=true; shift ;;
        *) 
            if [ -z "$IMAGE_ID" ]; then
                IMAGE_ID="$1"
            else
                echo "Error: Unexpected argument: $1"
                exit 1
            fi
            shift
            ;;
    esac
done

# Check if image ID is provided
if [ -z "$IMAGE_ID" ]; then
    echo "Usage: $0 [--local] [--estimate-cycles] <image_id>"
    echo "Example: $0 1f1af687201d7e5a4e930cbff67e90ba8dea06e993aaf90dfdf3cdda0dad31b9"
    exit 1
fi

# Source environment variables
ENV_FILE="$(dirname "$0")/../.env"
if [ -f "$ENV_FILE" ]; then
    echo "Loading environment variables from $ENV_FILE"
    set -a  # automatically export all variables
    source "$ENV_FILE"
    set +a
else
    echo "Warning: .env file not found at $ENV_FILE"
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

# Generate random seed for I Ching reading
echo "Generating random seed for I Ching reading..."
RANDOM_SEED=$(openssl rand -hex 32)

# Get RISC-V cycle estimate if requested
TIP_AMOUNT=12000  # Default tip amount
if [ "$ESTIMATE_CYCLES" = true ]; then
    echo "Estimating RISC-V cycles for ZK program..."
    ESTIMATE_OUTPUT=$("$BONSOL_CMD" estimate --manifest-path images/8bitoracle-iching/manifest.json)
    CYCLE_COUNT=$(echo "$ESTIMATE_OUTPUT" | grep -o '[0-9]*' | head -n1)
    if [ -n "$CYCLE_COUNT" ]; then
        echo "Estimated cycle count: $CYCLE_COUNT"
        # Note: We keep the tip amount fixed, cycle count is just for information
        echo "Using default tip amount: $TIP_AMOUNT"
    else
        echo "Warning: Could not parse cycle estimate"
        echo "Using default tip amount: $TIP_AMOUNT"
    fi
else
    echo "Using default tip amount: $TIP_AMOUNT"
fi

# Create input file
OUTPUT_PATH="images/8bitoracle-iching/input.json"
echo "Creating input file at $OUTPUT_PATH"

cat > "$OUTPUT_PATH" << EOF
{
  "imageId": "$IMAGE_ID",
  "executionConfig": {
    "verifyInputHash": false,
    "forwardOutput": true
  },
  "inputs": [
    {
      "inputType": "PublicUrl",
      "data": "$RANDOM_SEED"
    }
  ],
  "tip": $TIP_AMOUNT,
  "expiry": 1000
}
EOF

echo "Input file generated successfully!"
echo "Random seed used: $RANDOM_SEED"
echo "Tip amount set to: $TIP_AMOUNT"
if [ "$ESTIMATE_CYCLES" = true ] && [ -n "$CYCLE_COUNT" ]; then
    echo "Estimated RISC-V cycles: $CYCLE_COUNT"
fi
echo "You can now run 04-execute.sh to execute the program." 