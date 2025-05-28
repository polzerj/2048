//! User interface module for the 2048 game.
//!
//! This module provides renderers for displaying the game board in a terminal.
//! It includes both a colored renderer and a non-colored renderer for terminals
//! with limited color support.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::game::GameEngine;

/// Trait for rendering a game
pub trait GameRenderer {
    /// Render the game state as a vector of text lines
    fn render(&self, game: &dyn GameEngine) -> Vec<Line>;
    fn is_color(&self) -> bool {
        true // Default to color support
    }
}

/// Default renderer for the 2048 game
pub struct DefaultRenderer;

/// Get color for a number tile
pub fn get_color(num: u32) -> Color {
    match num {
        0 => Color::DarkGray,
        2 => Color::Green,
        4 => Color::Yellow,
        8 => Color::Blue,
        16 => Color::Magenta,
        32 => Color::Red,
        64 => Color::Cyan,
        128 => Color::LightGreen,
        256 => Color::LightYellow,
        512 => Color::LightBlue,
        1024 => Color::LightMagenta,
        2048 => Color::LightRed,
        _ => Color::LightCyan, // For larger numbers
    }
}

impl GameRenderer for DefaultRenderer {
    fn render(&self, game: &dyn GameEngine) -> Vec<Line> {
        let mut lines = vec![];
        lines.push(Line::from(
            "Score: ".to_string() + &game.score().to_string(),
        ));

        // Create a visual separator between score and board
        lines.push(Line::from(""));

        // For each row in the board, we'll create 3 lines to make square cells
        for row in game.board() {
            // Top border of the cells
            lines.push(Line::from(
                row.iter()
                    .map(|&num| Span::styled("┌─────┐ ", Style::default().fg(get_color(num))))
                    .collect::<Vec<Span>>(),
            ));

            // Cell content with the number
            lines.push(Line::from(
                row.iter()
                    .map(|&num| {
                        let content = if num == 0 {
                            "     ".to_string()
                        } else {
                            format!("{:^5}", num)
                        };
                        Span::styled(
                            format!("│{}│ ", content),
                            Style::default().fg(get_color(num)),
                        )
                    })
                    .collect::<Vec<Span>>(),
            ));

            // Bottom border of the cells
            lines.push(Line::from(
                row.iter()
                    .map(|&num| Span::styled("└─────┘ ", Style::default().fg(get_color(num))))
                    .collect::<Vec<Span>>(),
            ));
        }

        lines
    }
}

/// No-color renderer for the 2048 game (for terminals with limited color support)
pub struct NoColorRenderer;

impl GameRenderer for NoColorRenderer {
    fn render(&self, game: &dyn GameEngine) -> Vec<Line> {
        let mut lines = vec![];
        lines.push(Line::from(
            "Score: ".to_string() + &game.score().to_string(),
        ));

        // Create a visual separator between score and board
        lines.push(Line::from(""));

        // For each row in the board, we'll create 3 lines to make square cells
        for row in game.board() {
            // Top border of the cells
            lines.push(Line::from(
                row.iter()
                    .map(|_| Span::raw("┌─────┐ "))
                    .collect::<Vec<Span>>(),
            ));

            // Cell content with the number
            lines.push(Line::from(
                row.iter()
                    .map(|&num| {
                        let content = if num == 0 {
                            "     ".to_string()
                        } else {
                            format!("{:^5}", num)
                        };
                        Span::raw(format!("│{}│ ", content))
                    })
                    .collect::<Vec<Span>>(),
            ));

            // Bottom border of the cells
            lines.push(Line::from(
                row.iter()
                    .map(|_| Span::raw("└─────┘ "))
                    .collect::<Vec<Span>>(),
            ));
        }

        lines
    }

    fn is_color(&self) -> bool {
        false // No color support
    }
}
