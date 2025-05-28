//! 2048 game implementation in a terminal UI.
//!
//! This library provides modules for implementing a terminal-based
//! version of the popular 2048 game. It uses the ratatui library for
//! rendering the UI and crossterm for terminal manipulation.

// Game module containing the core game logic
pub mod game;
// UI module for handling rendering
pub mod ui;
// App module for handling application flow
pub mod app;
// Error handling module
pub mod error;

/// Reexported types to provide a cleaner API
pub mod prelude {
    pub use crate::app::App;
    pub use crate::error::{GameError, GameResult};
    pub use crate::game::{Game2048, GameEngine, MovementDirection};
    pub use crate::ui::{DefaultRenderer, GameRenderer, NoColorRenderer};
}