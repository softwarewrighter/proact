#!/bin/bash
# Run tests and clippy on all components
set -e

cd "$(dirname "$0")/.."

echo "Checking proact-cli..."
(cd components/proact-cli && cargo test && cargo clippy --all-targets --all-features -- -D warnings)

echo "Checking proact-templates..."
(cd components/proact-templates && cargo test && cargo clippy --all-targets --all-features -- -D warnings)

echo "Checking proact-metadata..."
(cd components/proact-metadata && cargo test && cargo clippy --all-targets --all-features -- -D warnings)

echo "Checking proact-run..."
(cd components/proact-run && cargo test && cargo clippy --all-targets --all-features -- -D warnings)

echo "All components passed checks"
