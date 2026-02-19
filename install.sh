#!/bin/bash
set -e

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case $ARCH in
x86_64)
  ARCH="x86_64"
  ;;
arm64 | aarch64)
  ARCH="arm64"
  ;;
*)
  echo "Unsupported architecture: $ARCH"
  exit 1
  ;;
esac

# Map OS names
case $OS in
linux)
  OS="linux"
  ;;
darwin)
  OS="macos"
  ;;
*)
  echo "Unsupported OS: $OS"
  exit 1
  ;;
esac

# Set variables
TOOL_NAME="quickcsv"
REPO="lukyuranek/quickcsv"
BINARY_NAME="${TOOL_NAME}-${OS}-${ARCH}"
INSTALL_DIR="/usr/local/bin"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}"

echo "Downloading ${TOOL_NAME} for ${OS}-${ARCH}..."

# Download binary
curl -L "${DOWNLOAD_URL}" -o "/tmp/${TOOL_NAME}"

# Make executable
chmod +x "/tmp/${TOOL_NAME}"

# Move to install directory
echo "Installing to ${INSTALL_DIR}/${TOOL_NAME}..."
if [ -w "$INSTALL_DIR" ]; then
  mv "/tmp/${TOOL_NAME}" "${INSTALL_DIR}/${TOOL_NAME}"
else
  sudo mv "/tmp/${TOOL_NAME}" "${INSTALL_DIR}/${TOOL_NAME}"
fi

echo "✓ ${TOOL_NAME} installed successfully!"
echo "Run '${TOOL_NAME} --help' to get started"
