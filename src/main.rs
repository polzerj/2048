//! Terminal-based 2048 game implementation using ratatui.
//!
//! This module contains the entry point for the TUI 2048 game.
//! It handles command line arguments, sets up the terminal environment,
//! and initializes the game components.

use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use tui_2048::app::App;
use tui_2048::game::Game2048;
use tui_2048::ui::{DefaultRenderer, NoColorRenderer};

fn main() -> Result<(), io::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut use_color = true;

    // Parse command line arguments
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

    // Use a closure to ensure terminal cleanup even in case of errors
    run_app(use_color)
}

/// Run the application with proper terminal setup and cleanup
fn run_app(use_color: bool) -> Result<(), io::Error> {
    // Setup terminal
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // Create game components
    let game = Game2048::default();

    // Use a result variable to store the application outcome
    let result = {
        // Create and run the app with the appropriate renderer
        if use_color {
            let renderer = DefaultRenderer;
            let mut app = App::new(game, renderer, terminal);
            app.run()
        } else {
            let renderer = NoColorRenderer;
            let mut app = App::new(game, renderer, terminal);
            app.run()
        }
    };

    // Ensure terminal state is restored even if there was an error
    let cleanup_result = restore_terminal(&mut stdout);

    // Return the application result or the cleanup error if that failed
    result.and(cleanup_result)
}

/// Restore terminal to its original state
fn restore_terminal(stdout: &mut io::Stdout) -> Result<(), io::Error> {
    crossterm::terminal::disable_raw_mode()?;

    // We need to create another terminal instance to show the cursor
    // since the original one is moved into the app
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.show_cursor()
}
