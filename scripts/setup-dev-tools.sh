#!/bin/bash
# FerrisDB Development Tools Setup Script
# Installs build and formatting dependencies used by <AgentName> and developers

set -e

echo "🔧 Installing FerrisDB development tools..."

# Ensure rustup is installed
if ! command -v rustup &> /dev/null; then
    echo "❌ rustup not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Install toolchain and components
rustup toolchain install stable
rustup component add rustfmt clippy

# Install cargo extensions if missing
if ! command -v cargo-watch &> /dev/null; then
    echo "➡️  Installing cargo-watch..."
    cargo install cargo-watch
fi

if ! command -v cargo-nextest &> /dev/null; then
    echo "➡️  Installing cargo-nextest..."
    cargo install cargo-nextest
fi

# Ensure npm is available for prettier
if ! command -v npm &> /dev/null; then
    echo "❌ npm not found. Install Node.js and npm from https://nodejs.org/"
    exit 1
fi

echo "➡️  Installing prettier..."
npm install -g prettier

echo "✅ Development tools installation complete!"
