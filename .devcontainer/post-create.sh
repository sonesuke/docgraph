#!/bin/bash
set -e

echo "Installing VSIX dependencies..."
npm install --prefix vsix
(cd vsix && npx vsce package -o ../docgraph.vsix)
if command -v code >/dev/null 2>&1; then
    code --install-extension docgraph.vsix
else
    echo "VS Code CLI (code) not found, skipping extension installation."
fi

echo "Installing cargo-binstall..."
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

echo "Installing cargo tools..."
rustup component add llvm-tools-preview
cargo binstall -y cargo-audit cargo-llvm-cov

echo "Checking project..."
cargo check

echo "Installing docgraph..."
cargo install --path . --force

echo "Installing claude..."
curl -fsSL https://claude.ai/install.sh | bash

echo "Configuring claude alias..."
echo 'alias claude="claude --plugin-dir /workspaces/docgraph/docgraph-plugin --allow-dangerously-skip-permissions"' >> $HOME/.bashrc
echo 'alias claude="claude --plugin-dir /workspaces/docgraph/docgraph-plugin --allow-dangerously-skip-permissions"' >> $HOME/.zshrc

echo "Authenticating claude..."
if [ -n "$Z_AI_API_KEY" ]; then
    npx -y @z_ai/coding-helper auth glm_coding_plan_global "$Z_AI_API_KEY" 
    npx -y @z_ai/coding-helper auth reload claude
fi

