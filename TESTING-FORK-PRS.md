# Testing Fork PRs

This document explains how to efficiently test the CI workflows for fork PRs without having to create actual PRs and waiting for the full CI process.

## The Problem

PRs from fork repositories have different permissions than PRs from the main repository:

1. They don't have write access to the GitHub Container Registry
2. They can't push Docker images to the registry
3. They need special handling for Docker image building and e2e tests

## Local Testing Script

The `test-fork-ci.sh` script simulates the CI workflow for fork PRs locally:

```bash
./test-fork-ci.sh
```

This script:

1. Authenticates with GitHub Container Registry (necessary to access base images)
2. Builds Docker images in the correct order
3. Verifies the images are available locally

## How to Test Changes to CI Workflows

1. Make changes to the workflow files
2. Test locally with `test-fork-ci.sh`
3. If the local test passes, create a PR and confirm it works with the real CI process

## Fork PR Workflow 

For fork PRs, the CI workflow:

1. Always logs in to GitHub Container Registry to access base images
2. Builds images with `load: true` instead of `push: true`
3. Uses the images locally in the Docker daemon for e2e tests

## Troubleshooting

If images fail to build:

1. Check authentication: 
   ```bash
   export GITHUB_TOKEN=your_token_here
   ```

2. Ensure base images are accessible:
   ```bash
   docker pull ghcr.io/bonsol-collective/bonsol-ci-env:1.0.0
   ```

3. Verify Docker engine is running:
   ```bash
   docker info
   ``` 
