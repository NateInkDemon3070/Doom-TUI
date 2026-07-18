use crate::config::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Launch,
    Wads,
    Mods,
    Engines,
    Settings,
}

impl Tab {
    pub fn all() -> &'static [Tab] {
        &[Tab::Launch, Tab::Wads, Tab::Mods, Tab::Engines, Tab::Settings]
    }

    pub fn label(&self) -> &str {
        match self {
            Tab::Launch => "Lanzar",
            Tab::Wads => "WADs",
            Tab::Mods => "Mods",
            Tab::Engines => "Engines",
            Tab::Settings => "Config",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Tab::Launch => 0,
            Tab::Wads => 1,
            Tab::Mods => 2,
            Tab::Engines => 3,
            Tab::Settings => 4,
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Tab::Launch,
            1 => Tab::Wads,
            2 => Tab::Mods,
            3 => Tab::Engines,
            4 => Tab::Settings,
            _ => Tab::Launch,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Insert,
    Confirm,
}

pub struct App {
    pub config: Config,
    pub tab: Tab,
    pub input_mode: InputMode,
    pub should_quit: bool,
    pub selected_wad: Option<usize>,
    pub selected_mods: Vec<bool>,
    pub mod_cursor: usize,
    pub wad_list: Vec<String>,
    pub mod_list: Vec<String>,
    pub scroll_offset: usize,
    pub confirm_action: Option<ConfirmAction>,
    pub status_msg: String,
    pub settings_cursor: usize,
    pub engine_cursor: usize,
    pub engine_edit_mode: bool,
    pub new_engine_name: String,
    pub new_engine_binary: String,
    pub new_path_buffer: String,
    pub path_edit_target: Option<PathEditTarget>,
    pub args_cursor: usize,
    pub args_editing: bool,
}

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    #[allow(dead_code)]
    LaunchGame,
    RemoveEngine(String),
    #[allow(dead_code)]
    SaveConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathEditTarget {
    Iwads,
    Mods,
}

impl App {
    pub fn new() -> Self {
        let config = Config::load();
        let wad_list = config.scan_wads();
        let mod_list = config.scan_mods();
        let mut app = App {
            config,
            tab: Tab::Launch,
            input_mode: InputMode::Normal,
            should_quit: false,
            selected_wad: None,
            selected_mods: vec![],
            mod_cursor: 0,
            wad_list,
            mod_list,
            scroll_offset: 0,
            confirm_action: None,
            status_msg: String::new(),
            settings_cursor: 0,
            engine_cursor: 0,
            engine_edit_mode: false,
            new_engine_name: String::new(),
            new_engine_binary: String::new(),
            new_path_buffer: String::new(),
            path_edit_target: None,
            args_cursor: 0,
            args_editing: false,
        };
        app.selected_mods = vec![false; app.mod_list.len()];
        app
    }

    pub fn refresh_lists(&mut self) {
        self.wad_list = self.config.scan_wads();
        self.mod_list = self.config.scan_mods();
        self.selected_mods = vec![false; self.mod_list.len()];
        self.selected_wad = None;
        self.mod_cursor = 0;
        self.scroll_offset = 0;
    }

    pub fn select_up(&mut self, current: Option<usize>, max: usize) -> Option<usize> {
        match current {
            None => {
                if max > 0 {
                    Some(max - 1)
                } else {
                    None
                }
            }
            Some(0) => Some(0),
            Some(i) => Some(i - 1),
        }
    }

    pub fn select_down(&mut self, current: Option<usize>, max: usize) -> Option<usize> {
        match current {
            None => {
                if max > 0 {
                    Some(0)
                } else {
                    None
                }
            }
            Some(i) => {
                if i + 1 < max {
                    Some(i + 1)
                } else {
                    Some(i)
                }
            }
        }
    }

    pub fn build_launch_command(&self) -> Vec<String> {
        let mut cmd = vec![self.config.active_engine_binary().to_string()];

        if let Some(wad_idx) = self.selected_wad {
            if let Some(wad) = self.wad_list.get(wad_idx) {
                cmd.push("-iwad".to_string());
                cmd.push(
                    self.config
                        .paths
                        .iwads
                        .join(wad)
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }

        for (i, selected) in self.selected_mods.iter().enumerate() {
            if *selected {
                if let Some(mod_name) = self.mod_list.get(i) {
                    cmd.push("-file".to_string());
                    cmd.push(
                        self.config
                            .paths
                            .mods
                            .join(mod_name)
                            .to_string_lossy()
                            .to_string(),
                    );
                }
            }
        }

        cmd.extend(self.config.engine_args.iter().cloned());

        cmd
    }
}
