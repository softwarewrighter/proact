#!/bin/bash
# Format all components
set -e

cd "$(dirname "$0")/.."

echo "Formatting proact-cli..."
(cd components/proact-cli && cargo fmt)

echo "Formatting proact-templates..."
(cd components/proact-templates && cargo fmt)

echo "Formatting proact-metadata..."
(cd components/proact-metadata && cargo fmt)

echo "Formatting proact-run..."
(cd components/proact-run && cargo fmt)

echo "All components formatted"
