#!/bin/bash
# Run proact CLI
set -e

cd "$(dirname "$0")/.."

BINARY="components/proact-cli/target/release/proact"

if [ ! -f "$BINARY" ]; then
    echo "Binary not found, building..."
    (cd components/proact-cli && cargo build --release)
fi

exec "$BINARY" "$@"
