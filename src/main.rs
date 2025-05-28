// src/main.rs
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use tui_2048::app::App;
use tui_2048::game::Game2048;
use tui_2048::ui::{DefaultRenderer, NoColorRenderer};

fn main() -> Result<(), io::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut use_color = true;

    for arg in &args[1..] {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("Usage: tui_2048 [OPTIONS]");
                println!("Options:");
                println!("  --help,     -h      Show this help message");
                println!("  --version,  -v      Show version information");
                println!("  --no-color          Run the game without colors");
                println!("\n----------------------- in-game controls ------------------------");
                println!("  Arrow keys or WASD to move tiles");
                println!("  U or Z to undo a move");
                println!("  R to restart after game over");
                println!("  Q or Esc to quit the game");
                return Ok(());
            }
            "--version" | "-v" => {
                println!("tui_2048 version 1.0.0");
                return Ok(());
            }
            "--no-color" => {
                use_color = false;
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

    // Create and run the app with the appropriate renderer
    let result = if use_color {
        let renderer = DefaultRenderer;
        let mut app = App::new(game, renderer, terminal);
        app.run()
    } else {
        let renderer = NoColorRenderer;
        let mut app = App::new(game, renderer, terminal);
        app.run()
    };

    // Restore terminal
    crossterm::terminal::disable_raw_mode()?;

    // We need to create another terminal instance to show the cursor
    // since the original one is moved into the app
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.show_cursor()?;

    result
}
