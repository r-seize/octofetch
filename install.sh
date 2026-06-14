#!/bin/sh
set -e

REPO           = "r-seize/octofetch"
BIN            = "octofetch"
INSTALL_DIR    = "/usr/local/bin"

# Detect OS and architecture
OS      = $(uname -s)
ARCH    = $(uname -m)

case "$OS" in
  Linux)
    case "$ARCH" in
      x86_64)  ARCHIVE    = "octofetch-linux-x86_64.tar.gz" ;;
      aarch64) ARCHIVE    = "octofetch-linux-arm64.tar.gz" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  Darwin)
    case "$ARCH" in
      x86_64)  ARCHIVE    = "octofetch-macos-intel.tar.gz" ;;
      arm64)   ARCHIVE    = "octofetch-macos-arm64.tar.gz" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS"
    echo "On Windows, download the .zip from the releases page."
    exit 1
    ;;
esac

# Get latest release tag
VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" \
  | grep '"tag_name"' | sed 's/.*"tag_name": *"\(.*\)".*/\1/')

if [ -z "$VERSION" ]; then
  echo "Failed to fetch latest version."
  exit 1
fi

URL="https://github.com/$REPO/releases/download/$VERSION/$ARCHIVE"

echo "Installing $BIN $VERSION ($OS/$ARCH)..."
curl -L "$URL" | tar xz

chmod +x "$BIN"

if [ -w "$INSTALL_DIR" ]; then
  mv "$BIN" "$INSTALL_DIR/$BIN"
else
  sudo mv "$BIN" "$INSTALL_DIR/$BIN"
fi

echo ""
echo "$BIN installed successfully!"
echo "Run: $BIN --help"
