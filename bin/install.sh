#!/bin/sh
# Ensure we're running in bash
if [ -z "$BASH_VERSION" ]; then
    exec bash "$0" "$@"
fi

# Ensure script is downloaded completely
{
    # Enable debug mode if DEBUG environment variable is set
    if [ ! -z "$DEBUG" ]; then
        set -x  # Print commands and their arguments as they are executed
    fi

    # Function to log debug messages
    debug_log() {
        if [ ! -z "$DEBUG" ]; then
            echo "DEBUG: $1" >&2
        fi
    }

    # Function to execute or simulate command based on DRY_RUN
    execute_cmd() {
        if [ ! -z "$DRY_RUN" ]; then
            echo "WOULD EXECUTE: $@"
        else
            "$@"
        fi
    }

    # Exit on any error
    set -e

    # Ensure we're in a suitable environment
    if [ ! -t 0 ] && [ -z "$NONINTERACTIVE" ]; then
        export NONINTERACTIVE=1
    fi

    debug_log "Script started"
    debug_log "Shell: $SHELL"
    debug_log "OSTYPE: $OSTYPE"
    debug_log "PWD: $(pwd)"
    debug_log "PATH: $PATH"

    debug_log "Installing RISC0 tools..."
    if [ ! -z "$DRY_RUN" ]; then
        echo "WOULD EXECUTE: curl -L https://risczero.com/install | bash"
        echo "WOULD EXECUTE: rzup install cargo-risczero 1.2.1"
    else
        # Download the install script to a temporary file first
        TEMP_SCRIPT=$(mktemp)
        curl -L https://risczero.com/install -o "$TEMP_SCRIPT"
        # Verify download was successful
        if [ $? -eq 0 ]; then
            bash "$TEMP_SCRIPT"
            rm -f "$TEMP_SCRIPT"
            rzup install cargo-risczero 1.2.1
        else
            echo "Failed to download RISC0 install script" >&2
            exit 1
        fi
    fi

    debug_log "Checking OS type and CUDA support..."
    # check os linux or mac
    if [ "$(uname)" = "Linux" ] || [[ "$OSTYPE" == "linux-gnu"* ]]; then
        debug_log "Linux OS detected"
        # check if nvidia-smi exists
        if ! command -v nvidia-smi &> /dev/null
        then
            debug_log "No CUDA support found"
            echo "installing without cuda support, proving will be slower"
            execute_cmd cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol 
        else
            debug_log "CUDA support found"
            echo "installing with cuda support"
            execute_cmd cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features linux
        fi
    elif [ "$(uname)" = "Darwin" ] || [[ "$OSTYPE" == "darwin"* ]]; then
        debug_log "MacOS detected"
        execute_cmd cargo install bonsol-cli --git https://github.com/bonsol-collective/bonsol --features mac
    else
        echo "Unsupported operating system" >&2
        exit 1
    fi

    debug_log "Script completed"
} # End of main script block

