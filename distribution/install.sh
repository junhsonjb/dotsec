#!/bin/sh
set -e

VERSION="v0.1.0"
BINARY_NAME="ds"
TARBALL_NAME="ds-x86_64-unknown-linux-gnu.tar.gz"
GITHUB_URL="https://github.com/junhsonjb/dotsec/releases/download/$VERSION/$TARBALL_NAME"

curl -sSL "$GITHUB_URL" -o "$TARBALL_NAME"

tar -xzf "$TARBALL_NAME"

sudo mv "$BINARY_NAME" /usr/local/bin
sudo chmod +x /usr/local/bin/"$BINARY_NAME"

echo "🏁 dotsec installed to /usr/local/bin/ds"
ds --version || echo "⛔️ something went wrong with running dotsec (binary: 'ds')"

rm "$TARBALL_NAME"
