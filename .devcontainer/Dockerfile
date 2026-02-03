FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

# Pre-build dependencies to cache them in the image
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Install Node.js and other dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    curl \
    git \
    pkg-config \
    libssl-dev

# Install Node.js LTS manually
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install -y nodejs

# Install vsce for VSIX packaging (global install needs root)
RUN npm install -g @vscode/vsce

# Ensure vscode user has ownership of cargo and rustup directories before switching
RUN chown -R vscode:vscode /usr/local/cargo /usr/local/rustup

USER vscode

# Install cargo-binstall as vscode
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install cargo tools and components as vscode
RUN rustup component add llvm-tools-preview \
    && cargo binstall -y cargo-audit cargo-llvm-cov

# Install Claude CLI as vscode
RUN curl -fsSL https://claude.ai/install.sh | bash
