#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "⚡ Launching Quick Hello World Application..."

if [ -f "$DIR/../../Cargo.toml" ]; then
    # Running inside Quick monorepo workspace
    cd "$DIR/../.."
    cargo run -p hello-world
else
    # Running as a standalone project
    cd "$DIR"
    cargo run
fi
