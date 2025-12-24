#!/usr/bin/env bash
set -e

# 1. Start Elasticsearch
echo "Starting Elasticsearch container..."
if docker compose version >/dev/null 2>&1; then
    docker compose -f docker/docker-compose.elasticsearch.yml up -d elasticsearch
else
    # Fallback for older docker-compose v1
    docker-compose -f docker/docker-compose.elasticsearch.yml up -d elasticsearch
fi

# 2. Generate Certs if missing
if [ ! -f "certs/server-cert.pem" ]; then
    echo "Certificates not found. Generating..."
    if [ -f "./bin/generate-certs.sh" ]; then
        chmod +x ./bin/generate-certs.sh
        ./bin/generate-certs.sh
    else
        echo "Error: bin/generate-certs.sh not found!"
        exit 1
    fi
fi

# 3. Run Bonfire
echo "Starting Bonfire Coordinator..."
# Network config
export WEBSOCKET_URL=ws://127.0.0.1:8900
# Security config
export TLS_CERT_FILE=certs/server-cert.pem
export TLS_KEY_FILE=certs/server-key.pem
# Storage config
export ELASTICSEARCH_URL="http://localhost:9200"
export RUST_LOG=info
cargo run --release -p bonsol-bonfire