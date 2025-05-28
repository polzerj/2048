use rand::prelude::*;

pub const SIZE: usize = 4;

/// Direction enum representing possible move directions
#[derive(Debug, Clone, Copy)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Trait defining the core game behavior
pub trait GameEngine {
    /// Move tiles in the specified direction
    fn move_in_direction(&mut self, direction: &MovementDirection) -> bool;

    /// Check if the game is over
    fn game_over(&self) -> bool;

    /// Get the current score
    fn score(&self) -> u32;

    /// Get the current board state
    fn board(&self) -> &[[u32; SIZE]; SIZE];
}

/// Implementation of the 2048 game
pub struct Game2048 {
    board: [[u32; SIZE]; SIZE],
    score: u32,
}

impl Game2048 {
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
        let mut rng = rand::rng();
        if let Some(&(i, j)) = empty.choose(&mut rng) {
            self.board[i][j] = if rng.random_bool(0.9) { 2 } else { 4 };
        }
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

    fn move_up(&mut self) -> bool {
        let mut moved = false;
        for j in 0..SIZE {
            let mut col: Vec<u32> = (0..SIZE).map(|i| self.board[i][j]).collect();
            moved |= self.merge(&mut col);
            for (i, &val) in col.iter().enumerate().take(SIZE) {
                moved |= self.board[i][j] != val;
                self.board[i][j] = val;
            }
        }
        moved
    }

    fn move_down(&mut self) -> bool {
        let mut moved = false;
        for j in 0..SIZE {
            let mut col: Vec<u32> = (0..SIZE).map(|i| self.board[SIZE - 1 - i][j]).collect();
            moved |= self.merge(&mut col);
            for (i, &val) in col.iter().enumerate().take(SIZE) {
                moved |= self.board[SIZE - 1 - i][j] != val;
                self.board[SIZE - 1 - i][j] = val;
            }
        }
        moved
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;
        for i in 0..SIZE {
            let mut row: Vec<u32> = self.board[i].to_vec();
            moved |= self.merge(&mut row);
            for (j, &val) in row.iter().enumerate().take(SIZE) {
                moved |= self.board[i][j] != val;
                self.board[i][j] = val;
            }
        }
        moved
    }

    fn move_right(&mut self) -> bool {
        let mut moved = false;
        for i in 0..SIZE {
            let mut row: Vec<u32> = self.board[i].iter().rev().cloned().collect();
            moved |= self.merge(&mut row);
            for (j, &val) in row.iter().enumerate().take(SIZE) {
                moved |= self.board[i][SIZE - 1 - j] != val;
                self.board[i][SIZE - 1 - j] = val;
            }
        }
        moved
    }
}

impl GameEngine for Game2048 {
    fn move_in_direction(&mut self, direction: &MovementDirection) -> bool {
        let moved = match direction {
            MovementDirection::Up => self.move_up(),
            MovementDirection::Down => self.move_down(),
            MovementDirection::Left => self.move_left(),
            MovementDirection::Right => self.move_right(),
        };

        if moved {
            self.spawn_tile();
            true
        } else {
            false
        }
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

    fn score(&self) -> u32 {
        self.score
    }

    fn board(&self) -> &[[u32; SIZE]; SIZE] {
        &self.board
    }
}

impl Default for Game2048 {
    fn default() -> Self {
        let mut game = Self {
            board: [[0; SIZE]; SIZE],
            score: 0,
        };
        game.spawn_tile();
        game.spawn_tile();
        game
    }
}
