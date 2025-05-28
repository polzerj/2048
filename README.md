# 2048
A simple 2048 TUI game written in Rust

## Installation
```bash
cargo install --git https://github.com/polzerj/2048
```

## Usage
```bash
tui_2048 [OPTIONS]
```

### Options
- `--help`, `-h` - Show help message
- `--version`, `-v` - Show version information
- `--no-color` - Run the game without colors

## Controls
- `w` or `↑` - Move Up
- `s` or `↓` - Move Down
- `a` or `←` - Move Left
- `d` or `→` - Move Right
- `u` or `z` - Undo last move
- `q` or `Esc` - Quit the game
- `r` - Restart (after game over)

## Features

- Full color terminal UI
- No-color mode for terminals with limited color support
- Simple keyboard controls (arrow keys or WASD)
- Undo functionality (up to 10 moves)
- Game over detection and restart option

## Code Structure

- `src/app.rs` - Application logic, handling input and drawing
- `src/game.rs` - Core game logic, board manipulation, moves and scoring
- `src/ui.rs` - Rendering logic for the game board
- `src/error.rs` - Custom error handling
- `src/main.rs` - Entry point, command line argument handling

## Development

Run tests:

```bash
cargo test
```

Run with additional debugging:

```bash
RUST_BACKTRACE=1 cargo run
```