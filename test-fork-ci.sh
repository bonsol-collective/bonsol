#!/bin/bash
set -e

echo "🧪 Testing Fork PR CI workflow locally"
echo "======================================"
echo "This script simulates how the CI workflow builds Docker images for fork PRs"

# Clean up any existing images
echo "🧹 Cleaning up existing test images..."
docker rmi -f bonsol-node-slim:latest bonsol-node-stark:latest bonsol-node-stark-cuda:latest 2>/dev/null || true

# Build the slim image first
echo "📦 Building base node slim image..."
docker build -t bonsol-node-slim:latest -f ./docker/Dockerfile.slim . || {
  echo "❌ Failed to build slim image"
  exit 1
}

# Build the stark image that depends on the slim image
echo "📦 Building stark image..."
docker build -t bonsol-node-stark:latest -f ./docker/Dockerfile.stark . \
  --build-arg IMAGE=bonsol-node-slim:latest || {
  echo "❌ Failed to build stark image"
  exit 1
}

# Build the full image that depends on the stark image
echo "📦 Building full cuda image..."
docker build -t bonsol-node-stark-cuda:latest -f ./docker/Dockerfile.full . \
  --build-arg IMAGE=bonsol-node-stark:latest || {
  echo "❌ Failed to build full image"
  exit 1
}

echo "✅ All images built successfully!"
echo "Docker images now available:"
docker images | grep bonsol-node

echo "This confirms the workflow will work for fork PRs" 
