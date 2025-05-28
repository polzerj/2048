//! Error handling for the 2048 game.
//!
//! This module provides a custom error type to better handle different error scenarios
//! in the application.

use std::fmt;
use std::io;

/// Custom error type for the 2048 game
#[derive(Debug)]
pub enum GameError {
    /// IO errors from terminal operations
    IoError(io::Error),
    /// Input parsing errors
    InputError(String),
    /// Game state errors
    GameStateError(String),
    /// Terminal setup errors
    TerminalError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::IoError(err) => write!(f, "IO error: {}", err),
            GameError::InputError(msg) => write!(f, "Input error: {}", msg),
            GameError::GameStateError(msg) => write!(f, "Game state error: {}", msg),
            GameError::TerminalError(msg) => write!(f, "Terminal error: {}", msg),
        }
    }
}

impl std::error::Error for GameError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GameError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for GameError {
    fn from(err: io::Error) -> Self {
        GameError::IoError(err)
    }
}

/// Result type alias for Game operations
pub type GameResult<T> = Result<T, GameError>;
