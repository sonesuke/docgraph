#!/bin/bash
set -e

echo "Installing VSIX dependencies..."
npm install --prefix vsix

echo "Installing cargo tools..."
cargo install --force cargo-audit cargo-llvm-cov

echo "Checking project..."
cargo check

echo "Installing docgraph..."
cargo install --force --path .
