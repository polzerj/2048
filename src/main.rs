// filepath: /home/polzerj/Documents/dev/rust/tui_2048/src/main.rs
// src/main.rs
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use tui_2048::app::App;
use tui_2048::game::Game2048;
use tui_2048::ui::DefaultRenderer;

fn main() -> Result<(), io::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    for arg in &args[1..] {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("Usage: tui_2048 [OPTIONS]");
                println!("Options:");
                println!("  --help,     -h      Show this help message");
                println!("  --version,  -v      Show version information");
                println!("\n----------------------- in-game controls ------------------------");
                println!("  Arrow keys or WASD to move tiles");
                println!("  Q or Esc to quit the game");
                return Ok(());
            }
            "--version" | "-v" => {
                println!("tui_2048 version 1.0.0");
                return Ok(());
            }
            _ => {
                eprintln!("Unknown option: {}", arg);
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid argument",
                ));
            }
        }
    }
    // Setup terminal
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // Create game components
    let game = Game2048::default();
    let renderer = DefaultRenderer;

    // Create and run the app
    let mut app = App::new(game, renderer, terminal);
    let result = app.run();

    // Restore terminal
    crossterm::terminal::disable_raw_mode()?;
    app.terminal.show_cursor()?;

    result
}
