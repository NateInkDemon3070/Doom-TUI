# doom-tui

A TUI launcher for Doom built in Rust. Supports multiple engines, WAD and mod selection, and keyboard/mouse navigation.

## Installation

```bash
git clone https://github.com/NateInkDemon3070/Doom-TUI.git
cd Doom-TUI
./install.sh
```

Requirements: [Rust](https://rustup.rs)

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
| `j/k` or `Up/Down` | Navigate list |
| `h/l` or `Left/Right` | Switch tab |
| `Tab` / `S-Tab` | Next/previous tab |
| `1-5` | Switch to tab directly |
| `Enter/Space` | Select/toggle |
| `i` | Edit (focus input) |
| `a` | Add (engine/arg) |
| `D` | Delete (engine/arg) |
| `r` | Refresh/rescan lists |
| `gg` | Go to top |
| `G` | Go to bottom |
| `Ctrl+d/u` | Scroll half page |
| `Ctrl+b/f` or `PageUp/Down` | Scroll full page |
| `q` | Quit (from Launch tab) or go back |
| `Esc` | Clear status |
| `?` | Show help |
| `Ctrl+c` | Quit |
| Mouse | Click to select |
## License

MIT
