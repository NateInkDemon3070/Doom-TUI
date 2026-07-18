use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// ============================================================
//  CONFIGURACION DOOM TUI LAUNCHER
//  Editá este archivo para personalizar todo.
//  Ruta: ~/.config/doom-tui/config.toml
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Configuración general
    pub general: GeneralConfig,
    /// Lista de engines de Doom instalados
    pub engines: Vec<Engine>,
    /// Rutas a carpetas de WADs y Mods
    pub paths: PathsConfig,
    /// Argumentos extra que se pasan al engine al lanzar
    #[serde(default)]
    pub engine_args: Vec<String>,
    /// Tema visual de la TUI
    #[serde(default)]
    pub theme: ThemeConfig,
    /// Imagen de splash (ruta a un .png/.jpg)
    #[serde(default)]
    pub splash_image: Option<String>,
    /// Atajos de teclado personalizados
    #[serde(default)]
    pub keybinds: KeybindsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Engine activo por defecto (nombre de alguno de la lista)
    #[serde(default = "default_engine")]
    pub active_engine: String,
}

fn default_engine() -> String {
    "uzdoom".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engine {
    /// Nombre visible (ej: "uzdoom", "gzdoom", "zandronum")
    pub name: String,
    /// Binario a ejecutar (ej: "uzdoom", "gzdoom")
    pub binary: String,
    /// Argumentos por defecto para este engine
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    /// Carpeta donde están tus WADs (iwads)
    pub iwads: PathBuf,
    /// Carpeta donde están tus Mods (pk3, wad, zip)
    pub mods: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Color de fondo: "black", "red", "green", "yellow", "blue", "magenta", "cyan", "gray", "darkgray"
    #[serde(default = "default_bg")]
    pub bg: String,
    /// Color principal (títulos, bordes)
    #[serde(default = "default_fg")]
    pub fg: String,
    /// Color de acento (selecciones, highlights)
    #[serde(default = "default_accent")]
    pub accent: String,
    /// Color de éxito/activo
    #[serde(default = "default_success")]
    pub success: String,
    /// Color de advertencia
    #[serde(default = "default_warning")]
    pub warning: String,
    /// Color de error/desactivado
    #[serde(default = "default_error")]
    pub error: String,
    /// Tipo de borde: "rounded", "plain", "double", "thick"
    #[serde(default = "default_border")]
    pub border_style: String,
    /// Mostrar imagen de splash en el tab de Lanzar
    #[serde(default = "default_true")]
    pub show_splash: bool,
}

fn default_bg() -> String { "black".to_string() }
fn default_fg() -> String { "cyan".to_string() }
fn default_accent() -> String { "yellow".to_string() }
fn default_success() -> String { "green".to_string() }
fn default_warning() -> String { "yellow".to_string() }
fn default_error() -> String { "red".to_string() }
fn default_border() -> String { "rounded".to_string() }
fn default_true() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindsConfig {
    /// Tecla para salir
    #[serde(default = "default_key_quit")]
    pub quit: String,
    /// Tecla para lanzar juego
    #[serde(default = "default_key_launch")]
    pub launch: String,
    /// Tecla para editar/activar
    #[serde(default = "default_key_edit")]
    pub edit: String,
    /// Tecla para agregar
    #[serde(default = "default_key_add")]
    pub add: String,
    /// Tecla para eliminar
    #[serde(default = "default_key_delete")]
    pub delete: String,
    /// Tecla para seleccionar
    #[serde(default = "default_key_select")]
    pub select: String,
    /// Tecla para ayuda
    #[serde(default = "default_key_help")]
    pub help: String,
}

fn default_key_quit() -> String { "q".to_string() }
fn default_key_launch() -> String { "g".to_string() }
fn default_key_edit() -> String { "e".to_string() }
fn default_key_add() -> String { "a".to_string() }
fn default_key_delete() -> String { "d".to_string() }
fn default_key_select() -> String { "enter".to_string() }
fn default_key_help() -> String { "?".to_string() }

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            bg: default_bg(),
            fg: default_fg(),
            accent: default_accent(),
            success: default_success(),
            warning: default_warning(),
            error: default_error(),
            border_style: default_border(),
            show_splash: true,
        }
    }
}

impl Default for KeybindsConfig {
    fn default() -> Self {
        KeybindsConfig {
            quit: default_key_quit(),
            launch: default_key_launch(),
            edit: default_key_edit(),
            add: default_key_add(),
            delete: default_key_delete(),
            select: default_key_select(),
            help: default_key_help(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        Config {
            general: GeneralConfig {
                active_engine: "uzdoom".to_string(),
            },
            engines: vec![
                Engine {
                    name: "uzdoom".to_string(),
                    binary: "uzdoom".to_string(),
                    args: vec![],
                },
                Engine {
                    name: "zdoom".to_string(),
                    binary: "zdoom".to_string(),
                    args: vec![],
                },
                Engine {
                    name: "zandronum".to_string(),
                    binary: "zandronum".to_string(),
                    args: vec![],
                },
                Engine {
                    name: "gzdoom".to_string(),
                    binary: "gzdoom".to_string(),
                    args: vec![],
                },
            ],
            paths: PathsConfig {
                iwads: home.join("Games/Doom/iwads"),
                mods: home.join("Games/Doom/mods"),
            },
            engine_args: vec![],
            theme: ThemeConfig::default(),
            splash_image: None,
            keybinds: KeybindsConfig::default(),
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("doom-tui/config.toml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            let contents = fs::read_to_string(&path).expect("No se pudo leer config.toml");
            toml::from_str(&contents).expect("config.toml malformado")
        } else {
            let config = Config::default();
            config.save();
            config
        }
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let contents = toml::to_string_pretty(self).expect("Error serializando config");
        fs::write(path, contents).expect("No se pudo guardar config.toml");
    }

    pub fn active_engine_binary(&self) -> &str {
        self.engines
            .iter()
            .find(|e| e.name == self.general.active_engine)
            .map(|e| e.binary.as_str())
            .unwrap_or("uzdoom")
    }

    pub fn scan_wads(&self) -> Vec<String> {
        scan_files(&self.paths.iwads, &[".wad"])
    }

    pub fn scan_mods(&self) -> Vec<String> {
        scan_files(&self.paths.mods, &[".pk3", ".wad", ".zip"])
    }

    pub fn add_engine(&mut self, name: String, binary: String) {
        if !self.engines.iter().any(|e| e.name == name) {
            self.engines.push(Engine {
                name,
                binary,
                args: vec![],
            });
        }
    }

    pub fn remove_engine(&mut self, name: &str) {
        self.engines.retain(|e| e.name != name);
        if self.general.active_engine == name {
            if let Some(first) = self.engines.first() {
                self.general.active_engine = first.name.clone();
            }
        }
    }

    /// Genera el archivo de configuración de ejemplo con comentarios
    pub fn generate_example_config() -> String {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        format!(
r#"# ================================================================
#  DOOM TUI LAUNCHER - Archivo de Configuracion
#  Ruta: ~/.config/doom-tui/config.toml
#
#  Editá este archivo con tu editor de texto favorito.
#  Los cambios se aplican al reiniciar el launcher.
# ================================================================

# --- Configuracion general ---
[general]
# Engine que se usa por defecto (debe coincidir con un nombre de la lista)
active_engine = "uzdoom"

# --- Engines instalados ---
# Agregá o quitá engines según lo que tengas instalado.
# name   = nombre visible en la TUI
# binary = comando para ejecutarlo en la terminal
# args   = argumentos extra por defecto para este engine
[[engines]]
name = "uzdoom"
binary = "uzdoom"
args = []

[[engines]]
name = "zdoom"
binary = "zdoom"
args = []

[[engines]]
name = "zandronum"
binary = "zandronum"
args = []

[[engines]]
name = "gzdoom"
binary = "gzdoom"
args = []

# --- Rutas a tus archivos ---
[paths]
# Carpeta donde guardás tus WADs (Doom2.wad, Freedoom, etc.)
iwads = "{{home}}/Games/Doom/iwads"

# Carpeta donde guardás tus Mods (Project Brutality, Brutal Doom, etc.)
mods = "{{home}}/Games/Doom/mods"

# --- Argumentos extra del engine ---
# Se agregan a todos los launches. Ejemplo: ["-fullscreen", "-vsync"]
engine_args = []

# --- Imagen de splash (Opcional) ---
# Poné la ruta a una imagen para que se muestre en el tab de Lanzar.
# Formatos soportados: PNG, JPG, BMP
# Si no ponés nada, no se muestra imagen.
# splash_image = "/home/usuario/imagenes/doom-splash.png"
splash_image = null

# --- Tema visual ---
# Personalizá los colores de la TUI.
# Colores disponibles: "black", "red", "green", "yellow", "blue",
#                      "magenta", "cyan", "gray", "darkgray"
[theme]
bg = "black"
fg = "cyan"
accent = "yellow"
success = "green"
warning = "yellow"
error = "red"

# Tipo de borde: "rounded", "plain", "double", "thick"
border_style = "rounded"

# Mostrar imagen de splash en el tab de Lanzar
show_splash = true

# --- Atajos de teclado personalizados ---
# Cambiá las teclas a tu gusto. Usá nombres de teclas de crossterm:
#   a-z, 0-9, enter, esc, space, tab, backspace,
#   up, down, left, right, f1-f12, etc.
[keybinds]
quit = "q"
launch = "g"
edit = "e"
add = "a"
delete = "d"
select = "enter"
help = "?"
"#
        ).replace("{home}", &home.to_string_lossy())
    }
}

fn scan_files(dir: &Path, extensions: &[&str]) -> Vec<String> {
    if !dir.exists() {
        return vec![];
    }
    let mut files: Vec<String> = fs::read_dir(dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_lowercase();
            extensions.iter().any(|ext| name.ends_with(ext))
        })
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    files.sort();
    files
}
