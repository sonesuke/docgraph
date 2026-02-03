FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

# Install Node.js and other dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    curl \
    git \
    pkg-config \
    libssl-dev

# Install Node.js LTS manually to avoid using features for faster builds
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install -y nodejs

# Install cargo-binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install vsce for VSIX packaging (global install needs root)
RUN npm install -g @vscode/vsce

# Install cargo tools and components as vscode
USER vscode
RUN rustup component add llvm-tools-preview \
    && cargo binstall -y cargo-audit cargo-llvm-cov

# Install Claude CLI as vscode
RUN curl -fsSL https://claude.ai/install.sh | bash
