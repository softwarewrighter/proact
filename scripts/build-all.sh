#!/bin/bash
# Build all components
set -e

cd "$(dirname "$0")/.."

echo "Building proact-cli..."
(cd components/proact-cli && cargo build --release)

echo "Building proact-templates..."
(cd components/proact-templates && cargo build)

echo "Building proact-metadata..."
(cd components/proact-metadata && cargo build)

echo "Building proact-run..."
(cd components/proact-run && cargo build)

echo "All components built successfully"
