#!/bin/bash

set -e  # Exit on any error

# Detect architecture
ARCH=$(uname -m)
case $ARCH in
    arm64|aarch64)
        BINARY="parallel-arm64"
        ;;
    x86_64|amd64)
        BINARY="parallel-amd64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Create temporary directory
TMP_DIR=$(mktemp -d)
cd $TMP_DIR

echo "Downloading parallel binary for $ARCH..."
curl -L -o parallel "https://github.com/arshankhanifar/salsa/releases/latest/download/$BINARY"

echo "Installing parallel..."
chmod +x parallel
if [ "$(id -u)" -eq 0 ]; then
    mv parallel /usr/local/bin/parallel
else
    sudo mv parallel /usr/local/bin/parallel
fi

echo "Cleaning up..."
cd - > /dev/null
rm -rf $TMP_DIR

echo "Installation complete! Try running: parallel --help"
