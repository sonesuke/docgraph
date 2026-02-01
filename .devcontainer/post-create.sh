#!/bin/bash
set -e

echo "Installing VSIX dependencies..."
npm install --prefix vsix

echo "Installing cargo-binstall..."
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

echo "Installing cargo tools..."
rustup component add llvm-tools-preview
cargo binstall -y cargo-audit cargo-llvm-cov

echo "Checking project..."
cargo check

echo "Installing docgraph..."
cargo install --path . --force
