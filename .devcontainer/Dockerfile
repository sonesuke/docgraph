FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

# Set CARGO_TARGET_DIR to a location that will be preserved in the image
ENV CARGO_TARGET_DIR=/usr/local/cargo/target

# Pre-build dependencies to cache them in the image
RUN mkdir -p /tmp/build/src /tmp/build/benches
WORKDIR /tmp/build
COPY Cargo.toml Cargo.lock ./
RUN echo "fn main() {}" > src/main.rs \
    && echo "fn main() {}" > benches/performance.rs \
    && cargo build --release \
    && cd / && rm -rf /tmp/build

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
