# doom-tui

Launcher TUI para Doom hecho en Rust con [ratatui](https://github.com/ratatui-org/ratatui). Soporta múltiples engines (uzdoom, gzdoom, zandronum), selección de WADs y mods, configuración vía TOML, y navegación con teclado/mouse.

## Instalación

```bash
git clone https://github.com/TU_USUARIO/doom-tui.git
cd doom-tui
./install.sh
```

Requisitos: [Rust](https://rustup.rs) (solo para compilar), ImageMagick (para el icono, opcional).

El script detecta automáticamente una imagen en `~/Descargas/` o `~/Downloads/` y la usa como icono del launcher.

## Uso

```bash
doom-tui              # Abrir el launcher
doom-tui --config     # Generar config de ejemplo
doom-tui --help       # Mostrar ayuda
```

## Configuración

El archivo de configuración se encuentra en `~/.config/doom-tui/config.toml`.

Se puede personalizar:
- **Engines**: nombre, binario y argumentos por defecto
- **Rutas**: carpetas de WADs y mods
- **Tema**: colores de fondo, texto, acento, bordes
- **Atajos**: teclas para salir, lanzar, editar, etc.
- **Args extra**: argumentos que se pasan al engine al lanzar

## Navegación

| Tecla | Acción |
|-------|--------|
| `h/l` o `flechas izq/der` | Cambiar de tab |
| `j/k` o `flechas arr/abajo` | Navegar lista |
| `Enter/Space` | Seleccionar/alternar |
| `e` | Editar (en Settings) |
| `a` | Agregar (engine/arg) |
| `d` | Eliminar (engine/arg) |
| `g` | Lanzar juego |
| `q` | Salir |
| `?` | Ayuda |
| `Ctrl+D/U` | Scroll rápido |
| Mouse | Click para seleccionar |

## Estructura

```
~/.config/doom-tui/config.toml                  # Configuración
~/.local/share/applications/doom-tui.desktop     # Archivo .desktop
~/.local/share/icons/hicolor/*/apps/doom-tui.png # Icono
```

## Licencia

MIT
