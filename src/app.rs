//! Application handling module.
//!
//! This module provides the main application logic for the 2048 game,
//! including key handling, drawing, and game state management.

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::error::GameResult;
use crate::game::{GameEngine, MovementDirection};
use crate::ui::GameRenderer;

/// Duration to wait for key events in the main game loop
const POLL_TIMEOUT: Duration = Duration::from_millis(500);

/// Duration to wait for key events in the game over screen
const GAME_OVER_POLL_TIMEOUT: Duration = Duration::from_millis(100);

/// The application state
pub struct App<'a, G: GameEngine + Default, R: GameRenderer> {
    game: G,
    renderer: R,
    pub terminal: Terminal<CrosstermBackend<&'a mut io::Stdout>>,
}

impl<'a, G: GameEngine + Default, R: GameRenderer> App<'a, G, R> {
    /// Create a new app instance
    pub fn new(
        game: G,
        renderer: R,
        terminal: Terminal<CrosstermBackend<&'a mut io::Stdout>>,
    ) -> Self {
        Self {
            game,
            renderer,
            terminal,
        }
    }

    /// Run the application
    pub fn run(&mut self) -> GameResult<()> {
        loop {
            self.draw()?;

            if event::poll(POLL_TIMEOUT)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Left | KeyCode::Char('a') => {
                            self.game.move_in_direction(&MovementDirection::Left);
                        }
                        KeyCode::Right | KeyCode::Char('d') => {
                            self.game.move_in_direction(&MovementDirection::Right);
                        }
                        KeyCode::Up | KeyCode::Char('w') => {
                            self.game.move_in_direction(&MovementDirection::Up);
                        }
                        KeyCode::Down | KeyCode::Char('s') => {
                            self.game.move_in_direction(&MovementDirection::Down);
                        }
                        KeyCode::Char('u') | KeyCode::Char('z') => {
                            self.game.undo();
                        }
                        _ => {}
                    }

                    if self.game.game_over() {
                        self.draw_game_over()?;

                        // Wait for a key press before quitting
                        loop {
                            if event::poll(GAME_OVER_POLL_TIMEOUT)? {
                                if let Event::Key(key) = event::read()? {
                                    match key.code {
                                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                                        KeyCode::Char('r') => {
                                            // Restart the game
                                            self.game = G::default();
                                            break;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Draw the game board
    fn draw(&mut self) -> GameResult<()> {
        self.terminal.draw(|f| {
            let size = f.area();
            let block = Block::default().title("2048").borders(Borders::ALL);
            let area = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size)[0];
            let para = Paragraph::new(self.renderer.render(&self.game)).block(block);
            f.render_widget(para, area);
        })?;
        Ok(())
    }

    /// Draw the game over screen
    fn draw_game_over(&mut self) -> GameResult<()> {
        self.terminal.draw(|f| {
            let size = f.area();
            let block = Block::default().title("Game Over!").borders(Borders::ALL);
            let area = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size)[0];

            let score = self.game.score();
            let game_over_text = format!(
                "\nFinal Score: {}\n\nPress 'r' to restart or 'q' to quit",
                score
            );

            let para =
                Paragraph::new(game_over_text)
                    .block(block)
                    .style(if self.renderer.is_color() {
                        Style::default().fg(Color::Red)
                    } else {
                        Style::default()
                    });
            f.render_widget(para, area);
        })?;
        Ok(())
    }
}
