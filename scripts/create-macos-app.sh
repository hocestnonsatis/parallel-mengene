#!/bin/bash

# Create macOS App Bundle for Parallel Mengene
# Usage: ./scripts/create-macos-app.sh <target> <artifact_name>

set -e

TARGET=$1
ARTIFACT_NAME=$2
BINARY_PATH="target/${TARGET}/release/parallel-mengene"
APP_NAME="Parallel Mengene"
APP_BUNDLE="${APP_NAME}.app"

if [ -z "$TARGET" ] || [ -z "$ARTIFACT_NAME" ]; then
    echo "Usage: $0 <target> <artifact_name>"
    echo "Example: $0 x86_64-apple-darwin macos-x86_64"
    exit 1
fi

if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    echo "Please build the project first with: cargo build --release --target $TARGET"
    exit 1
fi

echo "Creating macOS App Bundle for $ARTIFACT_NAME..."

# Clean up any existing app bundle
rm -rf "$APP_BUNDLE"

# Create app bundle structure
mkdir -p "${APP_BUNDLE}/Contents/MacOS"
mkdir -p "${APP_BUNDLE}/Contents/Resources"

# Copy binary
cp "$BINARY_PATH" "${APP_BUNDLE}/Contents/MacOS/"

# Create Info.plist
cat > "${APP_BUNDLE}/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>parallel-mengene</string>
    <key>CFBundleIdentifier</key>
    <string>com.parallel-mengene.cli</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleDisplayName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSRequiresAquaSystemAppearance</key>
    <false/>
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeName</key>
            <string>All Files</string>
            <key>CFBundleTypeRole</key>
            <string>Editor</string>
            <key>LSTypeIsPackage</key>
            <false/>
        </dict>
    </array>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
    <key>NSAppleScriptEnabled</key>
    <false/>
    <key>LSUIElement</key>
    <true/>
</dict>
</plist>
EOF

# Create a simple launcher script for GUI usage
cat > "${APP_BUNDLE}/Contents/MacOS/launcher.sh" << 'EOF'
#!/bin/bash
# Launcher script for Parallel Mengene GUI

# Get the directory where the app bundle is located
APP_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATH="$APP_DIR/parallel-mengene"

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    osascript -e 'display dialog "Parallel Mengene binary not found!" buttons {"OK"} default button "OK" with icon stop'
    exit 1
fi

# Open terminal and run the command
osascript << APPLESCRIPT
tell application "Terminal"
    activate
    do script "cd ~ && $BINARY_PATH --help && echo 'Press any key to close...' && read -n 1"
end tell
APPLESCRIPT
EOF

chmod +x "${APP_BUNDLE}/Contents/MacOS/launcher.sh"

# Create a simple README for the app
cat > "${APP_BUNDLE}/Contents/Resources/README.txt" << 'EOF'
Parallel Mengene - High-Performance Parallel File Compression Tool

This is a command-line tool. To use it:

1. Open Terminal
2. Navigate to this app bundle
3. Run: ./Contents/MacOS/parallel-mengene --help

Or double-click the app to open Terminal with the help command.

For more information, visit: https://github.com/hocestnonsatis/parallel-mengene
EOF

# Create dist directory and package
mkdir -p dist
tar -czf "dist/parallel-mengene-${ARTIFACT_NAME}.tar.gz" "$APP_BUNDLE"

echo "✅ macOS App Bundle created successfully!"
echo "📦 Package: dist/parallel-mengene-${ARTIFACT_NAME}.tar.gz"
echo "📱 App Bundle: $APP_BUNDLE"
