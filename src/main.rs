// filepath: /home/polzerj/Documents/dev/rust/tui_2048/src/main.rs
// src/main.rs
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};
use std::io;

use tui_2048::app::App;
use tui_2048::game::Game2048;
use tui_2048::ui::DefaultRenderer;

fn main() -> Result<(), io::Error> {
    // Setup terminal
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    
    // Create game components
    let game = Game2048::new();
    let renderer = DefaultRenderer::new();
    
    // Create and run the app
    let mut app = App::new(game, renderer, terminal);
    let result = app.run();
    
    // Restore terminal
    crossterm::terminal::disable_raw_mode()?;
    app.terminal.show_cursor()?;
    
    result
}
