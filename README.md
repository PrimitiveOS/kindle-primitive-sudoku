# Primitive Sudoku

Play fresh 9 × 9 Sudoku puzzles at three difficulty levels on your Kindle. Every generated puzzle has one solution.
Select a square and enter a number, or turn on **Notes** for pencil marks. **Check** marks mistakes, and **Undo** restores
the previous contents of a square. The app saves the active puzzle, pencil marks, and undo history.

## Develop locally

Run `cargo test --offline --workspace` and `cargo run --offline -p sudoku-desktop` from this directory, with the
Primitive Tools and Launcher repositories beside it. The desktop target runs sample tap interactions and writes
previews in `build/previews`. The Kindle app uses the launcher's drawing, input, rotation, and sleep handling.

See [INSTALL.md](INSTALL.md) and [device notes](docs/device-notes.md). `app.json` contains draft listing copy.
