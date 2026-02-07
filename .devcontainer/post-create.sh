#!/bin/bash
set -e

if [ -z "$CI" ]; then
    # Fix permissions for local development where CARGO_HOME is root-owned by the base image
    sudo chown -R vscode:vscode /usr/local/cargo

    echo "Installing VSIX dependencies..."
    npm install --prefix vsix
    (cd vsix && npx vsce package -o ../docgraph.vsix)
    if command -v code >/dev/null 2>&1 && code --version >/dev/null 2>&1; then
        echo "Installing docgraph.vsix..."
        code --install-extension docgraph.vsix
    else
        echo "VS Code CLI (code) not found or not working, skipping extension installation."
    fi

    echo "Checking project..."
    cargo check

    echo "Installing docgraph..."
    cargo install --path . --force

    echo "Configuring claude alias..."
    echo 'alias claude="claude --plugin-dir /workspaces/docgraph/docgraph-plugin --allow-dangerously-skip-permissions"' >> $HOME/.bashrc
    echo 'alias claude="claude --plugin-dir /workspaces/docgraph/docgraph-plugin --allow-dangerously-skip-permissions"' >> $HOME/.zshrc

    echo "Authenticating claude..."
    if [ -n "$Z_AI_API_KEY" ]; then
        npx -y @z_ai/coding-helper auth glm_coding_plan_global "$Z_AI_API_KEY" 
        npx -y @z_ai/coding-helper auth reload claude
    fi
else
    echo "Running in CI environment, skipping development setup..."
fi

