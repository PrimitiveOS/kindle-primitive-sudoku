use std::path::Path;

fn main() -> Result<(), String> {
    let dir = Path::new("/mnt/us/sudoku");
    primitive_app::run(sudoku_render::View::load(dir.join("puzzle.json")), dir)
}
