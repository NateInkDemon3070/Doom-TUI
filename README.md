# doom-tui

A TUI launcher for Doom built in Rust with [ratatui](https://github.com/ratatui-org/ratatui). Supports multiple engines (uzdoom, gzdoom, zandronum), WAD and mod selection, TOML configuration, and keyboard/mouse navigation.

## Installation

```bash
git clone https://github.com/NateInkDemon3070/Doom-TUI.git
cd Doom-TUI
./install.sh
```

Requirements: [Rust](https://rustup.rs) (only to compile), ImageMagick (optional, for icon resizing).

## Usage

```bash
doom-tui              # Open the launcher
doom-tui --config     # Generate example config
doom-tui --help       # Show help
```

## Configuration

The config file is located at `~/.config/doom-tui/config.toml`.

Customizable options:
- **Engines**: name, binary, and default arguments
- **Paths**: WAD and mod folders
- **Theme**: background, text, accent, and border colors
- **Keybinds**: keys for quit, launch, edit, etc.
- **Extra args**: arguments passed to the engine on launch

## Keybinds

| Key | Action |
|-----|--------|
| `h/l` or `Left/Right arrows` | Switch tab |
| `j/k` or `Up/Down arrows` | Navigate list |
| `Enter/Space` | Select/toggle |
| `e` | Edit (in Settings) |
| `a` | Add (engine/arg) |
| `d` | Remove (engine/arg) |
| `g` | Launch game |
| `q` | Quit |
| `?` | Help |
| `Ctrl+D/U` | Fast scroll |
| Mouse | Click to select |

## Structure

```
~/.config/doom-tui/config.toml                  # Configuration
~/.local/share/applications/doom-tui.desktop     # Desktop entry
~/.local/share/icons/hicolor/*/apps/doom-tui.png # Icon
```

## License

MIT
