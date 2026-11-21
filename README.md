<div align="center">

# Sudoku

**Sudoku for e-ink**

[![Release](https://img.shields.io/github/v/release/PrimitiveOS/kindle-primitive-sudoku?label=release)](https://github.com/PrimitiveOS/kindle-primitive-sudoku/releases) [![Licence: PolyForm-Noncommercial-1.0.0](https://img.shields.io/badge/licence-PolyForm--Noncommercial--1.0.0-blue)](LICENSE) [![Store downloads](https://img.shields.io/endpoint?url=https%3A%2F%2Fprimitiveos.github.io%2Fkindle-primitive-catalog%2Fv1%2Fbadges%2Fprimitive-sudoku.json)](https://primitiveos.github.io/kindle-primitive-catalog/v1/index.json)

[**Download**](https://github.com/PrimitiveOS/kindle-primitive-sudoku/releases/latest) · [**Install guide**](INSTALL.md) · [**Changelog**](CHANGELOG.md)

</div>

---

Generate a Sudoku puzzle with one solution. Choose a difficulty, add pencil marks, check the board for mistakes, and undo a move. Your current puzzle is saved.

## Quick start

After setting up PrimitiveOS, open **Store** from Primitive Launcher. Choose **Sudoku** under **Apps**,
tap **Get**, and open it from the Launcher grid. The [install guide](INSTALL.md) also covers the standalone USB package.

## Screenshots

<table>
  <tr>
    <td align="center"><img src="docs/images/sudoku.png" alt="Medium Sudoku puzzle with a number pad and game controls" width="260"><br><sub>A medium puzzle</sub></td>
    <td align="center"><img src="docs/images/sudoku-notes-dark.png" alt="Sudoku in dark mode with a pencil mark in the selected square" width="260"><br><sub>Pencil marks in dark mode</sub></td>
    <td align="center"><img src="docs/images/sudoku-new.png" alt="New puzzle dialog with Easy, Medium, and Hard choices" width="260"><br><sub>Choose a difficulty</sub></td>
  </tr>
</table>

<sub>Screenshots from the desktop simulator at 1264 × 1680, using sample puzzles.</sub>

## Highlights

- **Three difficulties.** Generate an Easy, Medium, or Hard puzzle.
- **Pencil marks.** Use Notes to add or remove candidates in an empty square.
- **Check and undo.** Mark incorrect entries with Check, or restore the previous contents of a square with Undo.
- **Saved progress.** Reopen your puzzle with its entries, notes, and undo history.

## Develop locally

Keep the Primitive Tools and Launcher repositories beside this checkout. The desktop command runs sample taps
and writes PNG previews to `build/previews`.

```sh
cargo test --offline --locked --workspace
cargo run --offline --locked -p sudoku-desktop
```

## Status

| Device | Status |
|---|---|
| Kindle Oasis 3 (1264 × 1680, firmware 5.18.2) | Tested on hardware |

**Current version: 0.1.1.** Changes are in [CHANGELOG.md](CHANGELOG.md).

## Downloads

The badge at the top shows the number of Store downloads. It counts GitHub downloads of this app's release packages, across all versions and architectures, and is refreshed daily from the [Store catalog](https://primitiveos.github.io/kindle-primitive-catalog/v1/index.json).

## Documentation

[Install guide](INSTALL.md) · [Changelog](CHANGELOG.md) · [Device notes](docs/device-notes.md)

## Licence

Copyright (C) 2026 Doan Nguyen.

- Original code is under the [PolyForm Noncommercial License 1.0.0](LICENSE).
- Original documentation and pictures, including these screenshots, are under [CC BY-NC-SA 4.0](LICENSE-DOCS).
- The Noto Sans fonts supplied by Primitive Launcher retain their SIL Open Font License.
