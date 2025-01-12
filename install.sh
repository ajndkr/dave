#!/bin/bash
set -e

# globals
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="devx"
REPO_OWNER="ajndkr"
REPO_NAME="devx"

# detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
    "linux")
        TARGET="x86_64-unknown-linux-musl"
        ;;
    "darwin")
        TARGET="x86_64-apple-darwin"
        ;;
    *)
        echo "Unsupported operating system: $OS"
        exit 1
        ;;
esac

# get latest release
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest" | grep '"tag_name":' | cut -d'"' -f4)
DOWNLOAD_URL="https://github.com/$REPO_OWNER/$REPO_NAME/releases/download/$LATEST_RELEASE/$BINARY_NAME-$TARGET"

echo "installing devx - version: $LATEST_RELEASE, build: $TARGET"

echo "downloading from: $DOWNLOAD_URL"
curl -L "$DOWNLOAD_URL" -o "$BINARY_NAME"
chmod +x "$BINARY_NAME"

# sudo mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
echo "devx successfully installed to $INSTALL_DIR/$BINARY_NAME"
echo "run 'devx help' to get started"
