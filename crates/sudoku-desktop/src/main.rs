use std::{
    fs,
    path::{Path, PathBuf},
};

use primitive_app::{font_paths, frame, write_png, App, Change, HEIGHT, WIDTH};
use primitive_ui::{Canvas, Input};
use sudoku_core::{Difficulty, Sudoku};
use sudoku_render::View;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .unwrap_or_else(|| "build/previews".into()),
    );
    fs::create_dir_all(&out)?;
    let fonts =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../doannt-kindle-launcher/assets/fonts");
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &font_paths(&fonts))?;
    let mut app = View::new(Sudoku::new(Difficulty::Medium, 42), None);
    write_png(
        &out.join("sudoku.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, false),
    )?;
    let (row, column) = (0..9)
        .flat_map(|row| (0..9).map(move |column| (row, column)))
        .find(|&(row, column)| !app.game().given(row, column))
        .unwrap();
    assert_eq!(
        app.input(Input::Tap(
            112.0 + column as f32 * 120.0,
            242.0 + row as f32 * 120.0
        )),
        Change::Partial
    );
    assert_eq!(app.input(Input::Tap(90.0, 1540.0)), Change::Partial);
    assert_eq!(app.input(Input::Tap(250.0, 1360.0)), Change::Partial);
    assert_eq!(app.game().notes(row, column), 1 << 2);
    write_png(
        &out.join("sudoku-notes-dark.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, true),
    )?;
    assert_eq!(app.input(Input::Tap(1050.0, 1555.0)), Change::Full);
    write_png(
        &out.join("sudoku-new.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, false),
    )?;
    Ok(())
}
