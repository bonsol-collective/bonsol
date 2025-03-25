#!/bin/bash
# Test script for fork PR CI workflow changes

set -euo pipefail

echo "Testing fork PR CI workflow changes"

# Check if we're already in a branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
  echo "Already on branch $CURRENT_BRANCH, please switch to main branch first"
  exit 1
fi

# Create a new test branch
TEST_BRANCH="test-fork-ci-workflow-$(date +%s)"
echo "Creating test branch: $TEST_BRANCH"
git checkout -b "$TEST_BRANCH"

# Make a small change to trigger CI
echo "# Test CI Workflow Change" >> README.md

# Commit the changes
git add README.md
git commit -m "test: Trigger CI workflow for fork PRs"

# Push the changes to your fork
echo "Pushing changes to your fork"
echo "This will create a PR that will run the updated CI workflow"
echo "The E2E tests should now use the pre-built image from GitHub Container Registry"
echo "instead of trying to build a new image for fork PRs"

# Provide instructions for manual steps
echo ""
echo "=== MANUAL STEPS ==="
echo "1. Push this branch to your fork:"
echo "   git push -u origin $TEST_BRANCH"
echo ""
echo "2. Create a Pull Request from your fork's $TEST_BRANCH branch to the main repository's main branch"
echo ""
echo "3. Monitor the CI workflow in GitHub Actions to verify that:"
echo "   - The call-build-node-container-image job runs but should not block E2E tests for fork PRs"
echo "   - The E2E Test (Fork PRs) job uses the pre-built image and runs successfully"
echo ""
echo "4. After testing, you can cleanup with:"
echo "   git checkout main"
echo "   git branch -D $TEST_BRANCH"
echo "   git push origin --delete $TEST_BRANCH (if needed)" 
