use crate::app::{App, InputMode, PathEditTarget, Tab};
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    draw_header(f, app, chunks[0]);
    draw_body(f, app, chunks[1]);
    draw_status_bar(f, app, chunks[2]);
}

fn parse_color(color_str: &str) -> Color {
    match color_str.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" => Color::DarkGray,
        "white" => Color::White,
        _ => Color::Cyan,
    }
}

fn parse_border(style: &str) -> BorderType {
    match style.to_lowercase().as_str() {
        "plain" => BorderType::Plain,
        "double" => BorderType::Double,
        "thick" => BorderType::Thick,
        _ => BorderType::Rounded,
    }
}

fn themed_border<'a>(app: &'a App, title: &str) -> Block<'a> {
    let t = &app.config.theme;
    Block::default()
        .borders(Borders::ALL)
        .border_type(parse_border(&t.border_style))
        .title(Span::styled(
            format!(" {} ", title),
            Style::default()
                .fg(parse_color(&t.fg))
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(Color::DarkGray))
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.config.theme;
    let titles: Vec<Line> = Tab::all()
        .iter()
        .map(|tab| {
            let style = if *tab == app.tab {
                Style::default()
                    .fg(parse_color(&t.accent))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Line::from(Span::styled(format!(" {} ", tab.label()), style))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(parse_border(&t.border_style))
                .title(Span::styled(
                    " DOOM TUI LAUNCHER ",
                    Style::default()
                        .fg(parse_color(&t.error))
                        .add_modifier(Modifier::BOLD),
                ))
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .select(app.tab.index())
        .highlight_style(
            Style::default()
                .fg(parse_color(&t.accent))
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

fn draw_body(f: &mut Frame, app: &App, area: Rect) {
    match app.tab {
        Tab::Launch => draw_launch_tab(f, app, area),
        Tab::Wads => draw_list_tab(f, app, area, &app.wad_list, "WADs", true),
        Tab::Mods => draw_list_tab(f, app, area, &app.mod_list, "Mods", false),
        Tab::Engines => draw_engines_tab(f, app, area),
        Tab::Settings => draw_settings_tab(f, app, area),
    }
}

fn draw_launch_tab(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.config.theme;

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(3)])
        .split(chunks[0]);

    // WAD seleccionado
    let wad_title = if let Some(idx) = app.selected_wad {
        app.wad_list
            .get(idx)
            .map(|s| format!(" WAD: {} ", s))
            .unwrap_or_else(|| " WAD: (ninguno) ".to_string())
    } else {
        " WAD: (ninguno - ve a la tab WADs) ".to_string()
    };

    let wad_block = Block::default()
        .borders(Borders::ALL)
        .border_type(parse_border(&t.border_style))
        .title(Span::styled(
            wad_title,
            Style::default()
                .fg(parse_color(&t.fg))
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(Color::DarkGray));

    let wad_text = if app.selected_wad.is_some() {
        vec![
            Line::from("WAD seleccionado OK"),
            Line::from(""),
            Line::from(Span::styled(
                "  Presiona Enter para lanzar",
                Style::default().fg(parse_color(&t.success)),
            )),
        ]
    } else {
        vec![
            Line::from(Span::styled(
                "  No hay WAD seleccionado",
                Style::default().fg(parse_color(&t.error)),
            )),
            Line::from(""),
            Line::from("  Presiona TAB -> WADs para elegir uno"),
        ]
    };

    let wad_para = Paragraph::new(wad_text)
        .block(wad_block)
        .alignment(Alignment::Center);
    f.render_widget(wad_para, left[0]);

    // Engine activo
    let engine_block = themed_border(app, "Engine");

    let engine_lines: Vec<Line> = app
        .config
        .engines
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let marker = if i == app.engine_cursor { ">>" } else { "  " };
            let style = if e.name == app.config.general.active_engine {
                Style::default()
                    .fg(parse_color(&t.success))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Line::from(Span::styled(format!("{} {}", marker, e.name), style))
        })
        .collect();

    let engine_para = Paragraph::new(engine_lines).block(engine_block);
    f.render_widget(engine_para, left[1]);

    // Panel derecho: Mods
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5)])
        .split(chunks[1]);

    // Mods seleccionados
    let selected_mods: Vec<Line> = app
        .mod_list
        .iter()
        .enumerate()
        .filter(|(i, _)| app.selected_mods.get(*i).copied().unwrap_or(false))
        .map(|(_, name)| {
            Line::from(Span::styled(
                format!(" * {}", name),
                Style::default().fg(parse_color(&t.success)),
            ))
        })
        .collect();

    let mods_display = if selected_mods.is_empty() {
        vec![
            Line::from(Span::styled(
                "  No hay mods seleccionados",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from("  Presiona TAB -> Mods para elegir"),
        ]
    } else {
        selected_mods
    };

    let mods_block = themed_border(
        app,
        &format!(
            "Mods seleccionados ({})",
            app.selected_mods.iter().filter(|x| **x).count()
        ),
    );

    let mods_para = Paragraph::new(mods_display).block(mods_block);
    f.render_widget(mods_para, right_chunks[0]);
}

fn draw_list_tab(f: &mut Frame, app: &App, area: Rect, list: &[String], title: &str, single_select: bool) {
    let t = &app.config.theme;
    let block = themed_border(app, &format!("{}s ({})", title, list.len()));

    if list.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("  No se encontraron {}s", title.to_lowercase()),
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  Presiona 'e' para cambiar la carpeta",
                Style::default().fg(parse_color(&t.accent)),
            )),
        ])
        .block(block)
        .alignment(Alignment::Center);
        f.render_widget(empty, area);
        return;
    }

    let selected = if single_select { app.selected_wad } else { None };

    let items: Vec<ListItem> = list
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let is_selected = if single_select {
                selected == Some(i)
            } else {
                app.selected_mods.get(i).copied().unwrap_or(false)
            };

            let marker = if is_selected { ">> " } else { "   " };
            let style = if is_selected {
                Style::default()
                    .fg(parse_color(&t.accent))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(Line::from(Span::styled(
                format!("{}{}", marker, name),
                style,
            )))
        })
        .collect();

    let list_widget = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let mut state = ListState::default();
    state.select(if single_select { app.selected_wad } else { Some(app.mod_cursor) });
    f.render_stateful_widget(list_widget, area, &mut state);
}

fn draw_engines_tab(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.config.theme;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(area);

    let block = themed_border(app, "Engines");

    let items: Vec<ListItem> = app
        .config
        .engines
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let is_active = e.name == app.config.general.active_engine;
            let is_cursor = i == app.engine_cursor;

            let style = if is_active {
                Style::default()
                    .fg(parse_color(&t.success))
                    .add_modifier(Modifier::BOLD)
            } else if is_cursor {
                Style::default().fg(parse_color(&t.accent))
            } else {
                Style::default().fg(Color::White)
            };

            let marker = if is_active { " [activo] " } else { "          " };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", e.name), style),
                Span::styled(marker, Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("({})", e.binary),
                    Style::default().fg(Color::Gray),
                ),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let mut state = ListState::default();
    state.select(Some(app.engine_cursor));
    f.render_stateful_widget(list, area, &mut state);

    let help = Paragraph::new(Line::from(vec![
        Span::styled("Enter", Style::default().fg(parse_color(&t.accent))),
        Span::raw(" activar  "),
        Span::styled("a", Style::default().fg(parse_color(&t.accent))),
        Span::raw(" agregar  "),
        Span::styled("d", Style::default().fg(parse_color(&t.accent))),
        Span::raw(" eliminar  "),
        Span::styled("j/k", Style::default().fg(parse_color(&t.accent))),
        Span::raw(" navegar"),
    ]))
    .block(themed_border(app, "Acciones"))
    .alignment(Alignment::Center);
    f.render_widget(help, chunks[1]);
}

fn draw_settings_tab(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.config.theme;
    let block = themed_border(app, "Configuracion");

    let active_engine = app.config.general.active_engine.clone();
    let iwads_path = app.config.paths.iwads.to_string_lossy().to_string();
    let mods_path = app.config.paths.mods.to_string_lossy().to_string();

    let extra_args = if app.config.engine_args.is_empty() {
        "(ninguno)".to_string()
    } else {
        app.config.engine_args.join(" ")
    };

    let mut lines = vec![
        setting_line(app, "Engine activo:", &active_engine, 0),
        setting_line(app, "Carpeta WADs:", &iwads_path, 1),
        setting_line(app, "Carpeta Mods:", &mods_path, 2),
        setting_line(app, "Args del engine:", &extra_args, 3),
    ];

    if app.input_mode == InputMode::Insert {
        if app.engine_edit_mode && app.tab == Tab::Settings {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("  > Engine activo: {}_", app.new_path_buffer),
                Style::default().fg(parse_color(&t.accent)),
            )));
        } else if app.engine_edit_mode && app.tab == Tab::Engines {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("  > Nombre: {}_", app.new_engine_name),
                Style::default().fg(parse_color(&t.accent)),
            )));
        } else {
            match app.path_edit_target {
                Some(PathEditTarget::Iwads) => {
                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled(
                        format!("  > Nueva ruta WADs: {}_", app.new_path_buffer),
                        Style::default().fg(parse_color(&t.accent)),
                    )));
                }
                Some(PathEditTarget::Mods) => {
                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled(
                        format!("  > Nueva ruta Mods: {}_", app.new_path_buffer),
                        Style::default().fg(parse_color(&t.accent)),
                    )));
                }
                None => {}
            }
        }
    }

    if app.args_editing {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  > Args: {}_", app.new_path_buffer),
            Style::default().fg(parse_color(&t.accent)),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  j/k/flechas: navegar  Enter/e: editar  a: agregar arg  d: borrar arg  mouse: click",
        Style::default().fg(Color::DarkGray),
    )));

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}

fn setting_line<'a>(app: &'a App, label: &'a str, value: &'a str, idx: usize) -> Line<'a> {
    let t = &app.config.theme;
    let is_cursor = idx == app.settings_cursor;
    let marker = if is_cursor { ">> " } else { "   " };

    let label_style = if is_cursor {
        Style::default()
            .fg(parse_color(&t.accent))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(parse_color(&t.fg))
    };

    Line::from(vec![
        Span::styled(marker, Style::default().fg(parse_color(&t.accent))),
        Span::styled(format!("{:<16}", label), label_style),
        Span::styled(value, Style::default().fg(Color::White)),
    ])
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.config.theme;
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(40)])
        .split(area);

    let mode_text = match app.input_mode {
        InputMode::Normal => " NORMAL ",
        InputMode::Insert => " INSERT ",
        InputMode::Confirm => " CONFIRMAR ",
    };

    let mode_style = match app.input_mode {
        InputMode::Normal => Style::default()
            .fg(Color::Black)
            .bg(parse_color(&t.success))
            .add_modifier(Modifier::BOLD),
        InputMode::Insert => Style::default()
            .fg(Color::Black)
            .bg(parse_color(&t.accent))
            .add_modifier(Modifier::BOLD),
        InputMode::Confirm => Style::default()
            .fg(Color::Black)
            .bg(parse_color(&t.error))
            .add_modifier(Modifier::BOLD),
    };

    let mode = Paragraph::new(Span::styled(mode_text, mode_style)).alignment(Alignment::Left);
    f.render_widget(mode, chunks[0]);

    let help = Paragraph::new(Span::styled(
        " q: salir  ?: ayuda  Tab/flechas: panel  mouse: ok",
        Style::default().fg(Color::DarkGray),
    ))
    .alignment(Alignment::Right);
    f.render_widget(help, chunks[1]);
}
