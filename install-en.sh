#!/bin/bash
set -e

BIN_NAME="doom-tui"
INSTALL_DIR="${HOME}/.cargo/bin"
CONFIG_DIR="${HOME}/.config/doom-tui"
DESKTOP_DIR="${HOME}/.local/share/applications"
ICON_DIR="${HOME}/.local/share/icons/hicolor"

echo "=== doom-tui installer ==="
echo ""

# Check Rust
if ! command -v cargo &>/dev/null; then
    echo "[!] Rust/Cargo not found. Install from https://rustup.rs"
    exit 1
fi

# Build
echo "[1/5] Building ${BIN_NAME}..."
cargo build --release --quiet

# Install binary
echo "[2/5] Installing binary to ${INSTALL_DIR}/"
mkdir -p "${INSTALL_DIR}"
cp "target/release/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"
chmod +x "${INSTALL_DIR}/${BIN_NAME}"

# Generate config
echo "[3/5] Generating config in ${CONFIG_DIR}/"
mkdir -p "${CONFIG_DIR}"
if [ ! -f "${CONFIG_DIR}/config.toml" ]; then
    "${INSTALL_DIR}/${BIN_NAME}" --config
    echo "  Example config created: ${CONFIG_DIR}/config.toml"
else
    echo "  config.toml already exists, skipping"
fi

# Install icon
echo "[4/5] Installing icon"
if [ -f "assets/doom-tui.png" ]; then
    mkdir -p "${ICON_DIR}/128x128/apps"
    mkdir -p "${ICON_DIR}/256x256/apps"
    cp "assets/doom-tui.png" "${ICON_DIR}/256x256/apps/${BIN_NAME}.png"
    if command -v magick &>/dev/null; then
        magick "assets/doom-tui.png" -resize 128x128 "${ICON_DIR}/128x128/apps/${BIN_NAME}.png"
    elif command -v convert &>/dev/null; then
        convert "assets/doom-tui.png" -resize 128x128 "${ICON_DIR}/128x128/apps/${BIN_NAME}.png"
    else
        cp "assets/doom-tui.png" "${ICON_DIR}/128x128/apps/${BIN_NAME}.png"
    fi
    echo "  Icon installed from: assets/doom-tui.png"
else
    echo "  No assets/doom-tui.png found, creating .desktop without icon"
fi

# Desktop file
echo "[5/5] Creating .desktop file"
mkdir -p "${DESKTOP_DIR}"
cat > "${DESKTOP_DIR}/${BIN_NAME}.desktop" << EOF
[Desktop Entry]
Name=Doom TUI
Comment=TUI launcher for Doom in Rust
Exec=kitty -e ${INSTALL_DIR}/${BIN_NAME}
Icon=${BIN_NAME}
Terminal=false
Type=Application
Categories=Game;Shooter;
Keywords=doom;tui;launcher;
EOF
echo "  ${DESKTOP_DIR}/${BIN_NAME}.desktop created"

echo ""
echo "=== Done! ==="
echo "  Binary:   ${INSTALL_DIR}/${BIN_NAME}"
echo "  Config:   ${CONFIG_DIR}/config.toml"
echo "  Desktop:  ${DESKTOP_DIR}/${BIN_NAME}.desktop"
echo ""
echo "  Run:      ${BIN_NAME}"
echo "  Or find it as 'Doom TUI' in your app launcher"
