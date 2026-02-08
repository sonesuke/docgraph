#!/bin/bash

if [ -z "$CI" ] && [ -z "$GITHUB_ACTIONS" ]; then
    # Fix permissions for local development where CARGO_HOME is root-owned by the base image
    sudo chown -R vscode:vscode /usr/local/cargo

    # Install Claude CLI as vscode user if not already installed
    if ! command -v claude >/dev/null 2>&1; then
        echo "[Devcontainer Setup] Installing Claude CLI..."
        curl -fsSL https://claude.ai/install.sh | bash

        # Add .local/bin to PATH for current session
        export PATH="$HOME/.local/bin:$PATH"

        # Add to shell configs for future sessions
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> $HOME/.bashrc
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> $HOME/.zshrc
    else
        echo "[Devcontainer Setup] Claude CLI already installed: $(claude --version)"
    fi

    echo "[Devcontainer Setup] Installing VSIX dependencies..."
    npm install --prefix vsix

    echo "[Devcontainer Setup] Packaging VSIX..."
    (cd vsix && npx vsce package -o ../docgraph.vsix)

    if command -v code >/dev/null 2>&1 && code --version >/dev/null 2>&1; then
        echo "[Devcontainer Setup] Installing docgraph.vsix..."
        code --install-extension docgraph.vsix
    else
        echo "[Devcontainer Setup] VS Code CLI (code) not found or not working, skipping extension installation."
    fi

    echo "[Devcontainer Setup] Checking project..."
    cargo check

    echo "[Devcontainer Setup] Installing docgraph..."
    cargo install --path . --force

    echo "[Devcontainer Setup] Configuring claude alias..."
    echo 'alias claude="claude --plugin-dir /workspaces/docgraph/docgraph-plugin --allow-dangerously-skip-permissions"' >> $HOME/.bashrc
    echo 'alias claude="claude --plugin-dir /workspaces/docgraph/docgraph-plugin --allow-dangerously-skip-permissions"' >> $HOME/.zshrc

    echo "[Devcontainer Setup] Authenticating claude..."
    if [ -n "$Z_AI_API_KEY" ]; then
        npx -y @z_ai/coding-helper auth glm_coding_plan_global "$Z_AI_API_KEY"
        npx -y @z_ai/coding-helper auth reload claude
    fi

    echo "[Devcontainer Setup] Complete!"
else
    echo "Running in CI environment, skipping development setup..."
fi
