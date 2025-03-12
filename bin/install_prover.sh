#!/usr/bin/env bash

set -e

DEFAULT_PROVER_PROVIDER_URL="https://risc0-prover-us-east-1-041119533185.s3.us-east-1.amazonaws.com"
DEFAULT_INSTALL_PREFIX="."
DEFAULT_JOB_TIMEOUT=3600
DEFAULT_VERSION="v2024-05-17.1"

# Fallback file sizes in bytes (used only if server doesn't provide size)
# Using regular arrays instead of associative arrays for compatibility with Bash 3.2
FALLBACK_FILES=("stark/rapidsnark" "stark/stark_verify" "stark/stark_verify_final.zkey" "stark/stark_verify.dat")
FALLBACK_SIZES=("2359296" "2359296" "3650722201" "52428800")

function get_fallback_size() {
    local file="$1"
    for i in "${!FALLBACK_FILES[@]}"; do
        if [ "${FALLBACK_FILES[$i]}" = "$file" ]; then
            echo "${FALLBACK_SIZES[$i]}"
            return 0
        fi
    done
    echo "0"
    return 1
}

function human_readable_size() {
    local bytes=$1
    if ((bytes < 1024)); then
        echo "${bytes}B"
    elif ((bytes < 1048576)); then
        echo "$(((bytes + 512) / 1024))KB"
    elif ((bytes < 1073741824)); then
        echo "$(((bytes + 524288) / 1048576))MB"
    else
        echo "$(((bytes + 536870912) / 1073741824))GB"
    fi
}

function get_remote_file_size() {
    local url="$1"
    local size
    size=$(curl -sI "$url" | grep -i content-length | awk '{print $2}' | tr -d '\r\n')
    if [ -n "$size" ]; then
        echo "$size"
        return 0
    else
        return 1
    fi
}

function verify_file_integrity() {
    local file="$1"
    local expected_size="$2"
    local actual_size

    if [ ! -f "$file" ]; then
        return 1
    fi

    actual_size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file")
    if [ "$actual_size" -eq "$expected_size" ]; then
        return 0
    else
        return 1
    fi
}

function download_with_resume() {
    local url="$1"
    local output_file="$2"
    local expected_size="$3"
    local temp_file="${output_file}.tmp"
    local retry_count=0
    local max_retries=3

    # Move existing incomplete download to temp file if it exists
    if [ -f "$output_file" ]; then
        mv "$output_file" "$temp_file"
    fi

    while [ $retry_count -lt $max_retries ]; do
        if [ -f "$temp_file" ]; then
            echo "Resuming previous download..."
            curl --max-time ${JOB_TIMEOUT} -C - --progress-bar -o "$temp_file" "$url"
        else
            curl --max-time ${JOB_TIMEOUT} --progress-bar -o "$temp_file" "$url"
        fi

        if verify_file_integrity "$temp_file" "$expected_size"; then
            mv "$temp_file" "$output_file"
            return 0
        else
            echo "Download incomplete or corrupted, retrying... ($(($retry_count + 1))/$max_retries)"
            ((retry_count++))
        fi
    done

    echo "Failed to download file after $max_retries attempts"
    rm -f "$temp_file"
    return 1
}

function parse_arguments() {
    # Initialize variables with default values
    PROVER_PROVIDER_URL="${DEFAULT_PROVER_PROVIDER_URL}"
    INSTALL_PREFIX="${DEFAULT_INSTALL_PREFIX}"
    JOB_TIMEOUT="${DEFAULT_JOB_TIMEOUT}"
    PROVER_VERSION="${DEFAULT_VERSION}"

    # Loop through all arguments
    while [[ "$#" -gt 0 ]]; do
        case "$1" in
        --help)
            echo "Usage: $0 [--prefix <install location>] [--prover-provider-url <prover provider URL>]"
            echo "Options:"
            echo "  --prefix                Specify the install location."
            echo "                          Default: $DEFAULT_INSTALL_PREFIX"
            echo "  --prover-provider-url   URL of the prover provider to install."
            echo "                          Default: $DEFAULT_PROVER_PROVIDER_URL"
            echo "  --job-timeout           Timeout for the job in seconds."
            echo "                          Default: $DEFAULT_JOB_TIMEOUT"
            echo ""
            echo "Minimum required disk space (using fallback sizes):"
            total_size=0
            for i in "${!FALLBACK_FILES[@]}"; do
                size="${FALLBACK_SIZES[$i]}"
                ((total_size += size))
            done
            echo "  Total: $(human_readable_size $total_size)"
            echo "  Note: Actual sizes may vary"
            exit 0
            ;;
        --prefix)
            shift
            if [[ -z "$1" ]]; then
                echo "Error: --prefix requires a non-empty argument."
                exit 1
            fi
            INSTALL_PREFIX="$1"
            ;;
        --prover-provider-url)
            shift
            if [[ -z "$1" ]]; then
                echo "Error: --prover-provider-url requires a non-empty argument."
                exit 1
            fi
            PROVER_PROVIDER_URL="$1"
            ;;
        --job-timeout)
            shift
            if [[ -z "$1" ]]; then
                echo "Error: --job-timeout requires a non-empty argument."
                exit 1
            fi
            JOB_TIMEOUT="$1"
            ;;
        --version)
            shift
            if [[ -z "$1" ]]; then
                echo "Error: --version requires a non-empty argument."
                exit 1
            fi
            PROVER_VERSION="$1"
            ;;
        *)
            echo "Error: Unknown option '$1'"
            echo "Use --help to see the usage."
            exit 1
            ;;
        esac
        shift
    done

    echo "PROVER_PROVIDER_URL is set to '$PROVER_PROVIDER_URL'"
    echo "INSTALL_PREFIX is set to '$INSTALL_PREFIX'"
    echo "JOB_TIMEOUT is set to '$JOB_TIMEOUT'"
}

if [ ! -x $(which curl) ]; then
    echo "Error: curl is required to download risc0-prover."
    exit 1
fi

parse_arguments "$@"

# Get actual file sizes from server and calculate total
# Using regular arrays instead of associative arrays
ACTUAL_FILES=()
ACTUAL_SIZES=()
total_size=0
echo "Checking file sizes on server..."
for stark_tech in "${FALLBACK_FILES[@]}"; do
    url="$PROVER_PROVIDER_URL/$PROVER_VERSION/$stark_tech"
    size=$(get_remote_file_size "$url")
    if [ -n "$size" ]; then
        ACTUAL_FILES+=("$stark_tech")
        ACTUAL_SIZES+=("$size")
        ((total_size += size))
        echo "✓ ${stark_tech}: $(human_readable_size $size)"
    else
        size=$(get_fallback_size "$stark_tech")
        ACTUAL_FILES+=("$stark_tech")
        ACTUAL_SIZES+=("$size")
        ((total_size += size))
        echo "! ${stark_tech}: $(human_readable_size $size) (using fallback size)"
    fi
done

echo
echo "Total download size will be approximately $(human_readable_size $total_size)"
echo "Please ensure you have enough disk space available."
echo

mkdir -p "${INSTALL_PREFIX}"/stark
for i in "${!ACTUAL_FILES[@]}"; do
    stark_tech="${ACTUAL_FILES[$i]}"
    expected_size="${ACTUAL_SIZES[$i]}"
    url="$PROVER_PROVIDER_URL/$PROVER_VERSION/$stark_tech"
    output_file="${INSTALL_PREFIX}/${stark_tech}"

    if [ -f "$output_file" ] && verify_file_integrity "$output_file" "$expected_size"; then
        echo "✓ ${stark_tech} already exists and is valid ($(human_readable_size $expected_size))"
        continue
    fi

    echo "Downloading ${stark_tech} (expected size: $(human_readable_size "$expected_size"))"
    if download_with_resume "$url" "$output_file" "$expected_size"; then
        echo "✓ ${stark_tech} downloaded successfully"
    else
        echo "❌ Failed to download ${stark_tech}"
        exit 1
    fi
done

chmod +x "${INSTALL_PREFIX}/stark/rapidsnark"
chmod +x "${INSTALL_PREFIX}/stark/stark_verify"

echo
echo "Installation complete! All files have been downloaded to ${INSTALL_PREFIX}/stark/"
