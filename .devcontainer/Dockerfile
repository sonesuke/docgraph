FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# Install Node.js and other dependencies as root
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

# Switch to vscode user
USER vscode

# Set CARGO_HOME to a location in user home to avoid permission issues in CI
# and add it to PATH
# ENV CARGO_HOME=/home/vscode/.cargo (Deferred until runtime)
ENV PATH=$CARGO_HOME/bin:$PATH
# Set CARGO_TARGET_DIR to a location in user home that is safe from volume mounts
ENV CARGO_TARGET_DIR=/home/vscode/.cargo-target

# No pre-build (cache is disabled by user request)

# Install cargo-binstall as vscode
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install cargo tools and components as vscode
RUN rustup component add llvm-tools-preview \
    && cargo binstall -y cargo-audit cargo-llvm-cov

# Install Claude CLI as vscode
RUN curl -fsSL https://claude.ai/install.sh | bash
