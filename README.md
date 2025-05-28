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
- `w` or $\uparrow$ - Move Up
- `s` or $\downarrow$ - Move Down
- `a` or $\leftarrow$ - Move Left
- `d` or $\rightarrow$ - Move Right
- `u` or `z` - Undo the last move
- `r` - Restart the game (after game over)
- `q` or `ESC` - Quit

## Features
- Classic 2048 gameplay
- Color-coded tiles for better visualization
- Undo functionality
- Game over screen with final score
- Restart option after game over
- No-color mode for terminals with limited color support