use crate::app::{App, ConfirmAction, InputMode, PathEditTarget, Tab};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use std::process::Command;

pub fn handle_events(app: &mut App) {
    match event::read().unwrap() {
        Event::Key(key) => match app.input_mode {
            InputMode::Normal => handle_normal(app, key),
            InputMode::Insert => handle_insert(app, key),
            InputMode::Confirm => handle_confirm(app, key),
        },
        Event::Mouse(mouse) => handle_mouse(app, mouse),
        _ => {}
    }
}

fn handle_normal(app: &mut App, key: KeyEvent) {
    match key.code {
        // Quit
        KeyCode::Char('q') => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                app.should_quit = true;
            } else {
                match app.tab {
                    Tab::Launch => app.should_quit = true,
                    _ => app.tab = Tab::Launch,
                }
            }
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }

        // Tab navigation: h/l, Left/Right, or 1-5
        KeyCode::Char('h') | KeyCode::Left => {
            let idx = app.tab.index();
            if idx > 0 {
                app.tab = Tab::from_index(idx - 1);
                app.scroll_offset = 0;
            }
        }
        KeyCode::Char('l') | KeyCode::Right => {
            let idx = app.tab.index();
            if idx < Tab::all().len() - 1 {
                app.tab = Tab::from_index(idx + 1);
                app.scroll_offset = 0;
            }
        }
        KeyCode::Char('1') => { app.tab = Tab::Launch; app.scroll_offset = 0; }
        KeyCode::Char('2') => { app.tab = Tab::Wads; app.scroll_offset = 0; }
        KeyCode::Char('3') => { app.tab = Tab::Mods; app.scroll_offset = 0; }
        KeyCode::Char('4') => { app.tab = Tab::Engines; app.scroll_offset = 0; }
        KeyCode::Char('5') => { app.tab = Tab::Settings; app.scroll_offset = 0; }

        // Navigation: j/k or Up/Down
        KeyCode::Char('j') | KeyCode::Down => handle_down(app),
        KeyCode::Char('k') | KeyCode::Up => handle_up(app),

        // Scroll: Ctrl+d / Ctrl+u
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            for _ in 0..5 {
                if app.tab == Tab::Mods {
                    let max = app.mod_list.len();
                    if max > 0 && app.mod_cursor + 1 < max {
                        app.mod_cursor += 1;
                        let visible = 20usize;
                        if app.mod_cursor >= app.scroll_offset + visible {
                            app.scroll_offset = app.mod_cursor.saturating_sub(visible - 1);
                        }
                    }
                } else {
                    handle_down(app);
                }
            }
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            for _ in 0..5 {
                if app.tab == Tab::Mods {
                    if app.mod_cursor > 0 {
                        app.mod_cursor -= 1;
                        if app.mod_cursor < app.scroll_offset {
                            app.scroll_offset = app.mod_cursor;
                        }
                    }
                } else {
                    handle_up(app);
                }
            }
        }

        // Select: Enter or Space
        KeyCode::Enter | KeyCode::Char(' ') => handle_select(app),

        // Tab-specific actions
        KeyCode::Char('e') => {
            if app.tab == Tab::Settings {
                match app.settings_cursor {
                    0 => {
                        // Change engine from settings
                        app.input_mode = InputMode::Insert;
                        app.new_path_buffer = app.config.general.active_engine.clone();
                        app.path_edit_target = None;
                        app.engine_edit_mode = true;
                        app.status_msg = "Escribi el nombre del engine:".to_string();
                    }
                    1 => {
                        app.input_mode = InputMode::Insert;
                        app.path_edit_target = Some(PathEditTarget::Iwads);
                        app.new_path_buffer = app.config.paths.iwads.to_string_lossy().to_string();
                    }
                    2 => {
                        app.input_mode = InputMode::Insert;
                        app.path_edit_target = Some(PathEditTarget::Mods);
                        app.new_path_buffer = app.config.paths.mods.to_string_lossy().to_string();
                    }
                    3 => {
                        app.input_mode = InputMode::Insert;
                        app.args_editing = true;
                        app.new_path_buffer = app.config.engine_args.join(" ");
                    }
                    _ => {}
                }
            } else if app.tab == Tab::Engines {
                if let Some(name) = app.config.engines.get(app.engine_cursor).map(|e| e.name.clone()) {
                    app.config.general.active_engine = name.clone();
                    app.status_msg = format!("Engine activo: {}", name);
                }
            }
        }

        // Launch
        KeyCode::Char('g') if app.tab == Tab::Launch => {
            launch_game(app);
        }

        // Engine tab: add/remove
        KeyCode::Char('a') if app.tab == Tab::Engines => {
            app.input_mode = InputMode::Insert;
            app.engine_edit_mode = true;
            app.new_engine_name.clear();
            app.new_engine_binary.clear();
            app.status_msg = "Nombre del engine:".to_string();
        }
        KeyCode::Char('d') if app.tab == Tab::Engines => {
            if let Some(engine) = app.config.engines.get(app.engine_cursor) {
                let name = engine.name.clone();
                app.confirm_action = Some(ConfirmAction::RemoveEngine(name));
                app.input_mode = InputMode::Confirm;
            }
        }

        // Settings: a to add arg
        KeyCode::Char('a') if app.tab == Tab::Settings => {
            app.input_mode = InputMode::Insert;
            app.args_editing = true;
            app.new_path_buffer.clear();
            app.status_msg = "Agregar argumento:".to_string();
        }

        // Settings: d to remove arg
        KeyCode::Char('d') if app.tab == Tab::Settings => {
            if !app.config.engine_args.is_empty() && app.args_cursor < app.config.engine_args.len() {
                app.config.engine_args.remove(app.args_cursor);
                if app.args_cursor >= app.config.engine_args.len() && app.args_cursor > 0 {
                    app.args_cursor -= 1;
                }
                app.status_msg = "Argumento eliminado".to_string();
            }
        }

        // Help
        KeyCode::Char('?') => {
            app.status_msg = "j/k/flechas:navegar  h/l/Tab:tabs  Enter:seleccionar  q:salir  e:editar  a:agregar  d:eliminar  g:lanzar  mouse:click".to_string();
        }

        _ => {}
    }
}

fn handle_insert(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.engine_edit_mode = false;
            app.path_edit_target = None;
            app.args_editing = false;
        }
        KeyCode::Enter => {
            if app.engine_edit_mode && app.tab == Tab::Settings {
                // Change active engine from settings
                let name = app.new_path_buffer.trim().to_string();
                if !name.is_empty() {
                    if app.config.engines.iter().any(|e| e.name == name) {
                        app.config.general.active_engine = name.clone();
                        app.status_msg = format!("Engine activo: {}", name);
                    } else {
                        app.status_msg = format!("Engine '{}' no encontrado", name);
                    }
                }
                app.engine_edit_mode = false;
                app.input_mode = InputMode::Normal;
            } else if app.engine_edit_mode && app.tab == Tab::Engines {
                if !app.new_engine_name.is_empty() && !app.new_engine_binary.is_empty() {
                    app.config.add_engine(app.new_engine_name.clone(), app.new_engine_binary.clone());
                    app.status_msg = format!("Engine '{}' agregado", app.new_engine_name);
                    app.engine_edit_mode = false;
                    app.input_mode = InputMode::Normal;
                }
            } else if let Some(target) = app.path_edit_target {
                let path = std::path::PathBuf::from(&app.new_path_buffer);
                match target {
                    PathEditTarget::Iwads => {
                        app.config.paths.iwads = path;
                        app.status_msg = "Carpeta WADs actualizada".to_string();
                    }
                    PathEditTarget::Mods => {
                        app.config.paths.mods = path;
                        app.status_msg = "Carpeta Mods actualizada".to_string();
                    }
                }
                app.path_edit_target = None;
                app.input_mode = InputMode::Normal;
                app.refresh_lists();
            } else if app.args_editing {
                let arg = app.new_path_buffer.trim().to_string();
                if !arg.is_empty() {
                    app.config.engine_args.push(arg);
                    app.status_msg = "Argumento agregado".to_string();
                }
                app.args_editing = false;
                app.input_mode = InputMode::Normal;
            }
        }
        KeyCode::Backspace => {
            app.new_path_buffer.pop();
        }
        KeyCode::Char(c) => {
            app.new_path_buffer.push(c);
        }
        _ => {}
    }
}

fn handle_confirm(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('s') => {
            if let Some(action) = app.confirm_action.take() {
                match action {
                    ConfirmAction::LaunchGame => {
                        launch_game(app);
                    }
                    ConfirmAction::RemoveEngine(name) => {
                        app.config.remove_engine(&name);
                        if app.engine_cursor >= app.config.engines.len() && app.engine_cursor > 0 {
                            app.engine_cursor -= 1;
                        }
                        app.status_msg = format!("Engine '{}' eliminado", name);
                    }
                    ConfirmAction::SaveConfig => {
                        app.config.save();
                        app.status_msg = "Config guardada".to_string();
                    }
                }
            }
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char('n') | KeyCode::Esc => {
            app.confirm_action = None;
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            let x = mouse.column;
            let y = mouse.row;

            // Header tabs area (row 0-2)
            if y <= 2 {
                let tab_width = 12u16;
                let start_x = 2u16;
                for (i, _) in Tab::all().iter().enumerate() {
                    let tab_x = start_x + (i as u16) * tab_width;
                    if x >= tab_x && x < tab_x + tab_width {
                        app.tab = Tab::from_index(i);
                        app.scroll_offset = 0;
                        break;
                    }
                }
                return;
            }

            // Body area (row 3+)
            match app.tab {
                Tab::Launch => {
                    // Left panel: WAD area or Engine area
                    // Right panel: Mods area
                    let half = app.config.paths.iwads.to_string_lossy().len() as u16 + 20;
                    if x < half {
                        // Could click on engine list
                    }
                }
                Tab::Wads => {
                    let list_start_y = 4u16;
                    let idx = (y as i32 - list_start_y as i32) as usize;
                    if idx < app.wad_list.len() {
                        app.selected_wad = Some(idx);
                        app.status_msg = format!("WAD: {}", app.wad_list[idx]);
                    }
                }
                Tab::Mods => {
                    let list_start_y = 4u16;
                    let idx = (y as i32 - list_start_y as i32) as usize + app.scroll_offset;
                    if idx < app.selected_mods.len() {
                        app.mod_cursor = idx;
                        app.selected_mods[idx] = !app.selected_mods[idx];
                        let name = &app.mod_list[idx];
                        let state = if app.selected_mods[idx] { "+" } else { "-" };
                        app.status_msg = format!("{} {}", state, name);
                    }
                }
                Tab::Engines => {
                    let list_start_y = 4u16;
                    let idx = (y as i32 - list_start_y as i32) as usize;
                    if idx < app.config.engines.len() {
                        app.engine_cursor = idx;
                        // Click to activate
                        let name = app.config.engines[idx].name.clone();
                        app.config.general.active_engine = name.clone();
                        app.status_msg = format!("Engine activo: {}", name);
                    }
                }
                Tab::Settings => {
                    let list_start_y = 4u16;
                    let idx = (y as i32 - list_start_y as i32) as usize;
                    if idx <= 3 {
                        app.settings_cursor = idx;
                        // Double-click effect: activate edit
                        match idx {
                            0 => {
                                app.input_mode = InputMode::Insert;
                                app.new_path_buffer = app.config.general.active_engine.clone();
                                app.engine_edit_mode = true;
                                app.status_msg = "Escribi el nombre del engine:".to_string();
                            }
                            1 => {
                                app.input_mode = InputMode::Insert;
                                app.path_edit_target = Some(PathEditTarget::Iwads);
                                app.new_path_buffer = app.config.paths.iwads.to_string_lossy().to_string();
                            }
                            2 => {
                                app.input_mode = InputMode::Insert;
                                app.path_edit_target = Some(PathEditTarget::Mods);
                                app.new_path_buffer = app.config.paths.mods.to_string_lossy().to_string();
                            }
                            3 => {
                                app.input_mode = InputMode::Insert;
                                app.args_editing = true;
                                app.new_path_buffer = app.config.engine_args.join(" ");
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        MouseEventKind::ScrollUp => {
            handle_up(app);
        }
        MouseEventKind::ScrollDown => {
            handle_down(app);
        }
        _ => {}
    }
}

fn handle_up(app: &mut App) {
    match app.tab {
        Tab::Wads => {
            app.selected_wad = app.select_up(app.selected_wad, app.wad_list.len());
        }
        Tab::Mods => {
            if app.mod_cursor > 0 {
                app.mod_cursor -= 1;
                // Keep cursor visible: scroll up if cursor goes above viewport
                if app.mod_cursor < app.scroll_offset {
                    app.scroll_offset = app.mod_cursor;
                }
            }
        }
        Tab::Engines => {
            if app.engine_cursor > 0 {
                app.engine_cursor -= 1;
            }
        }
        Tab::Settings => {
            if app.settings_cursor > 0 {
                app.settings_cursor -= 1;
            }
        }
        Tab::Launch => {}
    }
}

fn handle_down(app: &mut App) {
    match app.tab {
        Tab::Wads => {
            app.selected_wad = app.select_down(app.selected_wad, app.wad_list.len());
        }
        Tab::Mods => {
            let max = app.mod_list.len();
            if max > 0 && app.mod_cursor + 1 < max {
                app.mod_cursor += 1;
                // Keep cursor visible: we need to know visible rows, use a reasonable default
                // The actual visible rows will be computed in UI, but we can estimate ~20
                let visible = 20usize;
                if app.mod_cursor >= app.scroll_offset + visible {
                    app.scroll_offset = app.mod_cursor.saturating_sub(visible - 1);
                }
            }
        }
        Tab::Engines => {
            if app.engine_cursor + 1 < app.config.engines.len() {
                app.engine_cursor += 1;
            }
        }
        Tab::Settings => {
            if app.settings_cursor < 3 {
                app.settings_cursor += 1;
            }
        }
        Tab::Launch => {}
    }
}

fn handle_select(app: &mut App) {
    match app.tab {
        Tab::Launch => {
            if app.selected_wad.is_some() {
                launch_game(app);
            } else {
                app.status_msg = "Selecciona un WAD primero (tab 2)".to_string();
            }
        }
        Tab::Wads => {
            app.status_msg = if let Some(idx) = app.selected_wad {
                format!("WAD: {} seleccionado", app.wad_list[idx])
            } else {
                "Ningun WAD seleccionado".to_string()
            };
        }
        Tab::Mods => {
            let idx = app.mod_cursor;
            if idx < app.selected_mods.len() {
                app.selected_mods[idx] = !app.selected_mods[idx];
                let name = &app.mod_list[idx];
                let state = if app.selected_mods[idx] { "+" } else { "-" };
                app.status_msg = format!("{} {}", state, name);
            }
        }
        Tab::Engines => {
            if let Some(name) = app.config.engines.get(app.engine_cursor).map(|e| e.name.clone()) {
                app.config.general.active_engine = name.clone();
                app.status_msg = format!("Engine activo: {}", name);
            }
        }
        Tab::Settings => {
            match app.settings_cursor {
                0 => {
                    app.input_mode = InputMode::Insert;
                    app.new_path_buffer = app.config.general.active_engine.clone();
                    app.engine_edit_mode = true;
                    app.status_msg = "Escribi el nombre del engine:".to_string();
                }
                1 => {
                    app.input_mode = InputMode::Insert;
                    app.path_edit_target = Some(PathEditTarget::Iwads);
                    app.new_path_buffer = app.config.paths.iwads.to_string_lossy().to_string();
                }
                2 => {
                    app.input_mode = InputMode::Insert;
                    app.path_edit_target = Some(PathEditTarget::Mods);
                    app.new_path_buffer = app.config.paths.mods.to_string_lossy().to_string();
                }
                3 => {
                    app.input_mode = InputMode::Insert;
                    app.args_editing = true;
                    app.new_path_buffer = app.config.engine_args.join(" ");
                }
                _ => {}
            }
        }
    }
}

fn launch_game(app: &mut App) {
    let cmd = app.build_launch_command();
    if cmd.is_empty() || (cmd.len() == 1 && app.selected_wad.is_none()) {
        app.status_msg = "Selecciona un WAD primero".to_string();
        return;
    }

    app.config.save();

    let engine = cmd[0].clone();
    let args: Vec<String> = cmd[1..].to_vec();

    app.should_quit = true;

    eprintln!(" Ejecutando: {} {}", engine, args.join(" "));

    let status = Command::new(&engine)
        .args(&args)
        .status();

    match status {
        Ok(s) => {
            eprintln!(" Juego terminado (status: {})", s);
        }
        Err(e) => {
            eprintln!(" Error ejecutando {}: {}", engine, e);
            eprintln!(" Presiona Enter para continuar...");
            let _ = std::io::stdin().read_line(&mut String::new());
        }
    }
}
