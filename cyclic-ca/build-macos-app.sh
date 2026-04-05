#!/bin/bash
# Build macOS app bundle for Cyclic CA

set -e

VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')

echo "Building release binary (v${VERSION})..."
cargo build --release

echo "Creating app bundle..."
mkdir -p "CyclicCA.app/Contents/MacOS"
mkdir -p "CyclicCA.app/Contents/Resources"

cp target/release/cyclic-ca "CyclicCA.app/Contents/MacOS/"

cat > "CyclicCA.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>Cyclic Cellular Automata</string>
    <key>CFBundleDisplayName</key>
    <string>Cyclic Cellular Automata</string>
    <key>CFBundleIdentifier</key>
    <string>com.pedawson.cyclic-ca</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleExecutable</key>
    <string>cyclic-ca</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2026 pedawson</string>
</dict>
</plist>
EOF

echo "Done! App bundle created: CyclicCA.app (v${VERSION})"
echo "Copy to /Applications or run: open CyclicCA.app"
