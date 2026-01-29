#!/bin/bash
# Build macOS app bundle for Cyclic CA

set -e

echo "Building release binary..."
cargo build --release

echo "Creating app bundle..."
mkdir -p "CyclicCA.app/Contents/MacOS"
mkdir -p "CyclicCA.app/Contents/Resources"

cp target/release/cyclic-ca "CyclicCA.app/Contents/MacOS/"

echo "Done! App bundle created at: CyclicCA.app"
echo "You can copy it to /Applications or run: open CyclicCA.app"
