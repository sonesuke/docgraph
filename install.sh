#!/bin/bash
set -e

OWNER="sonesuke"
REPO="docgraph"
BINARY_NAME="docgraph"

# Detect OS and Arch
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
    x86_64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

case "$OS" in
    linux) TARGET="${ARCH}-unknown-linux-gnu" ;;
    darwin) TARGET="${ARCH}-apple-darwin" ;;
    *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

echo "Detecting latest version..."
LATEST_TAG=$(curl -s "https://api.github.com/repos/$OWNER/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    echo "Failed to fetch latest version."
    exit 1
fi

echo "Downloading $BINARY_NAME $LATEST_TAG for $TARGET..."
ASSET_NAME="${BINARY_NAME}-${TARGET}.tar.gz"
DOWNLOAD_URL="https://github.com/$OWNER/$REPO/releases/download/$LATEST_TAG/$ASSET_NAME"

TEMP_DIR=$(mktemp -d)
curl -L "$DOWNLOAD_URL" -o "$TEMP_DIR/$ASSET_NAME"
tar -xzf "$TEMP_DIR/$ASSET_NAME" -C "$TEMP_DIR"

echo "Installing to /usr/local/bin..."
sudo mv "$TEMP_DIR/$BINARY_NAME" /usr/local/bin/
sudo chmod +x /usr/local/bin/$BINARY_NAME

rm -rf "$TEMP_DIR"
echo "Successfully installed $BINARY_NAME $LATEST_TAG"
