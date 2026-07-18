mod app;
mod config;
mod events;
mod ui;

use app::App;
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // --config: genera un archivo de configuración de ejemplo
    if args.iter().any(|a| a == "--config" || a == "-c") {
        let example = config::Config::generate_example_config();
        let path = config::Config::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, &example)?;
        eprintln!(" Config de ejemplo generado en: {}", path.display());
        eprintln!(" Editá ese archivo y reiniciá el launcher.");
        return Ok(());
    }

    // --help
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(" doom-tui - Launcher TUI para Doom");
        eprintln!();
        eprintln!(" Uso:");
        eprintln!("   doom-tui              Abrir el launcher");
        eprintln!("   doom-tui --config     Generar config de ejemplo");
        eprintln!("   doom-tui --help       Mostrar esta ayuda");
        eprintln!();
        eprintln!(" Config: ~/.config/doom-tui/config.toml");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if app.should_quit {
            break;
        }

        events::handle_events(&mut app);
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
