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

sudo mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
echo "devx successfully installed to $INSTALL_DIR/$BINARY_NAME"
echo "run 'devx help' to get started"

# setup zsh completion
read -p "setup zsh completions? (y/n) " setup_completions

if [ "$setup_completions" = "y" ]; then
   COMPLETION_DIR="${HOME}/.zsh/completions"
   mkdir -p "$COMPLETION_DIR"
   "$INSTALL_DIR/$BINARY_NAME" --generate zsh > "$COMPLETION_DIR/_devx"

   if ! grep -q "fpath=($COMPLETION_DIR \$fpath)" ~/.zshrc; then
       echo "" >> ~/.zshrc
       echo "# zsh completions" >> ~/.zshrc
       echo "fpath=($COMPLETION_DIR \$fpath)" >> ~/.zshrc
       echo "autoload -U compinit && compinit" >> ~/.zshrc
       echo "restart shell or run 'source ~/.zshrc' to enable completions"
   else
       echo "updating completions, run 'source ~/.zshrc' to refresh"
   fi
fi
