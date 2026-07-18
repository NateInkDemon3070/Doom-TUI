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
    echo "[!] Rust/Cargo no encontrado. Instalalo desde https://rustup.rs"
    exit 1
fi

# Build
echo "[1/5] Compilando ${BIN_NAME}..."
cargo build --release --quiet

# Install binary
echo "[2/5] Instalando binario en ${INSTALL_DIR}/"
mkdir -p "${INSTALL_DIR}"
cp "target/release/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"
chmod +x "${INSTALL_DIR}/${BIN_NAME}"

# Generate config
echo "[3/5] Generando config en ${CONFIG_DIR}/"
mkdir -p "${CONFIG_DIR}"
if [ ! -f "${CONFIG_DIR}/config.toml" ]; then
    "${INSTALL_DIR}/${BIN_NAME}" --config
    echo "  Config de ejemplo creado: ${CONFIG_DIR}/config.toml"
else
    echo "  config.toml ya existe, no se sobreescribe"
fi

# Install icon
echo "[4/5] Instalando icono"
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
    echo "  Icono instalado desde: assets/doom-tui.png"
else
    echo "  No se encontro assets/doom-tui.png, se crea .desktop sin icono"
fi

# Desktop file
echo "[5/5] Creando archivo .desktop"
mkdir -p "${DESKTOP_DIR}"
cat > "${DESKTOP_DIR}/${BIN_NAME}.desktop" << EOF
[Desktop Entry]
Name=Doom TUI
Comment=Launcher TUI para Doom en Rust
Exec=kitty -e ${INSTALL_DIR}/${BIN_NAME}
Icon=${BIN_NAME}
Terminal=false
Type=Application
Categories=Game;Shooter;
Keywords=doom;tui;launcher;
EOF
echo "  ${DESKTOP_DIR}/${BIN_NAME}.desktop creado"

echo ""
echo "=== Listo! ==="
echo "  Binario:  ${INSTALL_DIR}/${BIN_NAME}"
echo "  Config:   ${CONFIG_DIR}/config.toml"
echo "  Desktop:  ${DESKTOP_DIR}/${BIN_NAME}.desktop"
echo ""
echo "  Ejecuta:  ${BIN_NAME}"
echo "  O buscalo como 'Doom TUI' en tu launcher de apps"
