// filepath: /home/polzerj/Documents/dev/rust/tui_2048/src/game.rs
use rand::prelude::*;

pub const SIZE: usize = 4;
pub const UNDO_LIMIT: usize = 10; // Limit for undo history

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

    /// Undo the last move if possible
    fn undo(&mut self) -> bool;
}

/// Implementation of the 2048 game
pub struct Game2048 {
    board: [[u32; SIZE]; SIZE],
    score: u32,
    previous_states: Vec<([[u32; SIZE]; SIZE], u32)>, // Store previous (board, score) pairs
}

impl Game2048 {
    /// Save the current game state before making changes
    fn save_state(&mut self) {
        self.previous_states.push((self.board, self.score));
        // Limit history size to prevent excessive memory usage
        if self.previous_states.len() > UNDO_LIMIT {
            self.previous_states.remove(0);
        }
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
        // Save the current state before the move
        self.save_state();

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
            // If no tiles moved, we don't need to keep this state
            self.previous_states.pop();
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

    fn undo(&mut self) -> bool {
        if let Some((prev_board, prev_score)) = self.previous_states.pop() {
            self.board = prev_board;
            self.score = prev_score;
            true
        } else {
            false
        }
    }
}

impl Default for Game2048 {
    fn default() -> Self {
        let mut game = Self {
            board: [[0; SIZE]; SIZE],
            score: 0,
            previous_states: Vec::new(),
        };
        game.spawn_tile();
        game.spawn_tile();
        game
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_initialization() {
        let game = Game2048::default();
        assert_eq!(game.score(), 0);
        let empty_tiles: usize = game.board().iter().flatten().filter(|&&x| x == 0).count();
        assert_eq!(empty_tiles, SIZE * SIZE - 2); // Two tiles should be spawned
    }

    #[test]
    fn test_move_left() {
        let mut game = Game2048::default();
        game.board = [[2, 2, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        game.move_in_direction(&MovementDirection::Left);
        let expected = [[4, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        fix_gen(&mut game, &expected);
        assert_eq!(game.board, expected);
    }

    #[test]
    fn test_move_right() {
        let mut game = Game2048::default();
        game.board = [
            [0, 0, 16, 64],
            [4, 0, 4, 0],
            [16, 0, 0, 16],
            [2048, 0, 0, 16],
        ];
        game.move_in_direction(&MovementDirection::Right);
        let expected = [
            [0, 0, 16, 64],
            [0, 0, 0, 8],
            [0, 0, 0, 32],
            [0, 0, 2048, 16],
        ];
        fix_gen(&mut game, &expected);
        assert_eq!(game.board, expected);
    }

    #[test]
    fn test_move_up() {
        let mut game = Game2048::default();
        game.board = [[2, 0, 0, 0], [2, 0, 0, 0], [4, 0, 0, 0], [8, 0, 0, 0]];
        game.move_in_direction(&MovementDirection::Up);
        let expected = [[4, 0, 0, 0], [4, 0, 0, 0], [8, 0, 0, 0], [0, 0, 0, 0]];
        fix_gen(&mut game, &expected);
        assert_eq!(game.board, expected);
    }

    #[test]
    fn test_move_down() {
        let mut game = Game2048::default();
        game.board = [[0, 0, 0, 0], [2, 0, 0, 0], [2, 0, 0, 0], [4, 0, 0, 0]];
        game.move_in_direction(&MovementDirection::Down);
        let expected = [[0, 0, 0, 0], [0, 0, 0, 0], [4, 0, 0, 0], [4, 0, 0, 0]];
        fix_gen(&mut game, &expected);
        assert_eq!(game.board, expected);
    }

    #[test]
    fn test_game_over() {
        let mut game = Game2048::default();
        game.board = [[2, 8, 4, 16], [8, 2, 16, 4], [32, 4, 2, 32], [2, 16, 32, 2]];
        assert!(game.game_over());
    }

    fn fix_gen(game: &mut Game2048, expected: &[[u32; 4]; 4]) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if game.board[i][j] != expected[i][j]
                    && expected[i][j] == 0
                    && (game.board[i][j] == 2 || game.board[i][j] == 4)
                {
                    game.board[i][j] = 0; // Ignore generated tiles
                    return;
                }
            }
        }
        assert!(false, "Board does not have a generated value");
    }
}
