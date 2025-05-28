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

use crate::game::{GameEngine, MovementDirection};
use crate::ui::GameRenderer;

/// The application state
pub struct App<'a, G: GameEngine, R: GameRenderer> {
    game: G,
    renderer: R,
    pub terminal: Terminal<CrosstermBackend<&'a mut io::Stdout>>,
}

impl<'a, G: GameEngine, R: GameRenderer> App<'a, G, R> {
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
    pub fn run(&mut self) -> Result<(), io::Error> {
        loop {
            self.draw()?;

            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(key) = event::read()? {
                    if let Some(dir) = match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => Some(MovementDirection::Left),
                        KeyCode::Right => Some(MovementDirection::Right),
                        KeyCode::Up => Some(MovementDirection::Up),
                        KeyCode::Down => Some(MovementDirection::Down),
                        _ => None,
                    } {
                        self.game.move_in_direction(&dir);

                        if self.game.game_over() {
                            self.draw_game_over()?;
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    /// Draw the game board
    fn draw(&mut self) -> Result<(), io::Error> {
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
    fn draw_game_over(&mut self) -> Result<(), io::Error> {
        self.terminal.draw(|f| {
            let size = f.area();
            let block = Block::default().title("Game Over").borders(Borders::ALL);
            let area = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size)[0];
            let para = Paragraph::new("No more moves left!")
                .block(block)
                .style(Style::default().fg(Color::Red));
            f.render_widget(para, area);
        })?;
        Ok(())
    }
}
