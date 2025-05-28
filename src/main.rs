// src/main.rs
use crossterm::event::{self, Event, KeyCode};
use rand::seq::IndexedRandom;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::{io, time::Duration};

const SIZE: usize = 4;

struct Game2048 {
    board: [[u32; SIZE]; SIZE],
    score: u32,
}

enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

fn get_color(num: u32) -> Color {
    match num {
        0 => Color::DarkGray,
        2 => Color::Green,
        4 => Color::LightYellow,
        8 => Color::Blue,
        16 => Color::Magenta,
        32 => Color::Red,
        64 => Color::Cyan,
        _ => Color::LightCyan, // For larger numbers
    }
}

impl Game2048 {
    fn new() -> Self {
        let mut game = Self {
            board: [[0; SIZE]; SIZE],
            score: 0,
        };
        game.spawn_tile();
        game.spawn_tile();
        game
    }

    fn spawn_tile(&mut self) {
        let empty: Vec<(usize, usize)> = self
            .board
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &val)| val == 0)
                    .map(move |(j, _)| (i, j))
            })
            .collect();
        empty.choose(&mut rand::rng()).map(|&(i, j)| {
            self.board[i][j] = if rand::random_bool(0.9) { 2 } else { 4 };
        });
    }

    fn move_in_direction(&mut self, direction: &MovementDirection) -> bool {
        match direction {
            MovementDirection::Up => self.move_up(),
            MovementDirection::Down => self.move_down(),
            MovementDirection::Left => self.move_left(),
            MovementDirection::Right => self.move_right(),
        }
        .then(|| {
            self.spawn_tile();
            true
        })
        .unwrap_or(false)
    }

    fn game_over(&self) -> bool {
        // Check if there are any empty spaces or possible merges
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j] == 0 {
                    return false; // Found an empty space
                }
                if j < SIZE - 1 && self.board[i][j] == self.board[i][j + 1] {
                    return false; // Found a horizontal merge
                }
                if i < SIZE - 1 && self.board[i][j] == self.board[i + 1][j] {
                    return false; // Found a vertical merge
                }
            }
        }
        true // No moves left
    }

    fn move_up(&mut self) -> bool {
        let mut moved = false;
        for j in 0..SIZE {
            let mut col: Vec<u32> = (0..SIZE).map(|i| self.board[i][j]).collect();
            moved |= self.merge(&mut col);
            for i in 0..SIZE {
                moved |= self.board[i][j] != col[i];
                self.board[i][j] = col[i];
            }
        }
        moved
    }
    fn move_down(&mut self) -> bool {
        let mut moved = false;
        for j in 0..SIZE {
            let mut col: Vec<u32> = (0..SIZE).map(|i| self.board[SIZE - 1 - i][j]).collect();
            moved |= self.merge(&mut col);
            for i in 0..SIZE {
                moved |= self.board[SIZE - 1 - i][j] != col[i];
                self.board[SIZE - 1 - i][j] = col[i];
            }
        }
        moved
    }
    fn move_left(&mut self) -> bool {
        let mut moved = false;
        for i in 0..SIZE {
            let mut row: Vec<u32> = self.board[i].to_vec();
            moved |= self.merge(&mut row);
            for j in 0..SIZE {
                moved |= self.board[i][j] != row[j];
                self.board[i][j] = row[j];
            }
        }
        moved
    }
    fn move_right(&mut self) -> bool {
        let mut moved = false;
        for i in 0..SIZE {
            let mut row: Vec<u32> = self.board[i].iter().rev().cloned().collect();
            moved |= self.merge(&mut row);
            for j in 0..SIZE {
                moved |= self.board[i][SIZE - 1 - j] != row[j];
                self.board[i][SIZE - 1 - j] = row[j];
            }
        }
        moved
    }

    fn merge(&mut self, line: &mut Vec<u32>) -> bool {
        let mut moved = false;
        let mut i = 0;
        while i < line.len() {
            if line[i] == 0 {
                i += 1;
                continue;
            }
            let mut j = i + 1;
            while j < line.len() && line[j] == 0 {
                j += 1;
            }
            if j < line.len() && line[i] == line[j] {
                self.score += line[i];
                line[i] *= 2;
                line[j] = 0;
                moved = true;
            }
            i += 1;
        }
        // Compact the line
        let mut new_line: Vec<u32> = line.iter().filter(|&&x| x != 0).cloned().collect();
        new_line.resize(SIZE, 0);
        *line = new_line;
        moved
    }

    fn render(&self) -> Vec<Line> {
        let mut lines = vec![];
        lines.push(Line::from("Score: ".to_string() + &self.score.to_string()));

        // Create a visual separator between score and board
        lines.push(Line::from(""));

        // For each row in the board, we'll create 3 lines to make square cells
        for row in &self.board {
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

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    let mut game = Game2048::new();

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default().title("2048").borders(Borders::ALL);
            let area = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size)[0];
            let para = Paragraph::new(game.render()).block(block);
            f.render_widget(para, area);
        })?;

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if let Some(dir) = match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => Some(MovementDirection::Left),
                    KeyCode::Right => Some(MovementDirection::Right),
                    KeyCode::Up => Some(MovementDirection::Up),
                    KeyCode::Down => Some(MovementDirection::Down),
                    _ => None,
                } {
                    game.move_in_direction(&dir);

                    if game.game_over() {
                        draw_game_over(&mut terminal)?;
                        break;
                    }
                }
            }
        }
    }
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn draw_game_over(
    terminal: &mut Terminal<CrosstermBackend<&mut io::Stdout>>,
) -> Result<(), io::Error> {
    terminal.draw(|f| {
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
