use std::{
    fs, io,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use primitive_app::{App, Change, WIDTH};
use primitive_ui::{
    back_button, header, screen_rect, Canvas, Input, Rect, TextStyle, BLACK, CAPTION, HAIRLINE,
    PAPER, SECONDARY,
};
use sudoku_core::{Difficulty, Sudoku};

const BOARD_X: i32 = 92;
const BOARD_Y: i32 = 222;
const CELL: i32 = 120;
const PUZZLE: Rect = Rect::new(242, 535, 780, 555);

pub struct View {
    game: Sudoku,
    data: Option<PathBuf>,
    selected: Option<(usize, usize)>,
    notes: bool,
    checked: bool,
    new_game: bool,
    save_error: bool,
}

impl View {
    pub fn new(game: Sudoku, data: Option<PathBuf>) -> Self {
        Self {
            game,
            data,
            selected: None,
            notes: false,
            checked: false,
            new_game: false,
            save_error: false,
        }
    }

    pub fn load(data: PathBuf) -> Self {
        let game = fs::read(&data)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Sudoku>(&bytes).ok())
            .filter(Sudoku::is_valid)
            .unwrap_or_else(|| Sudoku::new(Difficulty::Easy, seed()));
        Self::new(game, Some(data))
    }

    pub fn game(&self) -> &Sudoku {
        &self.game
    }

    fn record(&mut self) {
        self.save_error = self.save().is_err();
    }

    fn draw_board(&self, canvas: &mut Canvas, w: i32) {
        for row in 0..9 {
            for column in 0..9 {
                let cell = tile(w, row, column);
                let given = self.game.given(row, column);
                let selected = self.selected == Some((row, column));
                let digit = self.game.value(row, column);
                if selected {
                    canvas.fill(cell.inset(canvas.px(5)), BLACK);
                } else if given {
                    canvas.fill(cell.inset(canvas.px(2)), 247);
                }
                let ink = if selected { PAPER } else { BLACK };
                if digit != 0 {
                    let style = TextStyle::new(62).center().ink(ink);
                    let style = if given { style.bold() } else { style };
                    canvas.text(
                        Rect::new(cell.x, cell.y + canvas.px(17), cell.w, cell.h),
                        &digit.to_string(),
                        style,
                    );
                    if self.checked && self.game.mistake(row, column) {
                        canvas.text(
                            Rect::new(
                                cell.right() - canvas.px(35),
                                cell.y + canvas.px(8),
                                canvas.px(30),
                                canvas.px(35),
                            ),
                            "×",
                            TextStyle::new(30).bold().center().ink(ink),
                        );
                    }
                } else {
                    let notes = self.game.notes(row, column);
                    for mark in 1..=9 {
                        if notes & (1 << mark) != 0 {
                            let x = cell.x + canvas.px(8 + 35 * ((mark - 1) % 3));
                            let y = cell.y + canvas.px(10 + 32 * ((mark - 1) / 3));
                            canvas.text(
                                Rect::new(x, y, canvas.px(35), canvas.px(33)),
                                &mark.to_string(),
                                TextStyle::new(24).center().ink(ink),
                            );
                        }
                    }
                }
            }
        }
        for index in 0..=9 {
            let (weight, ink) = if index % 3 == 0 {
                (5, BLACK)
            } else {
                (2, HAIRLINE)
            };
            canvas.fill(
                screen_rect(
                    w,
                    BOARD_X + index * CELL,
                    BOARD_Y,
                    weight,
                    9 * CELL + weight,
                ),
                ink,
            );
            canvas.fill(
                screen_rect(
                    w,
                    BOARD_X,
                    BOARD_Y + index * CELL,
                    9 * CELL + weight,
                    weight,
                ),
                ink,
            );
        }
    }
}

fn seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

fn tile(w: i32, row: usize, column: usize) -> Rect {
    screen_rect(
        w,
        BOARD_X + column as i32 * CELL,
        BOARD_Y + row as i32 * CELL,
        CELL,
        CELL,
    )
}

fn digit_key(w: i32, digit: u8) -> Rect {
    screen_rect(w, 40 + (i32::from(digit) - 1) * 132, 1323, 124, 100)
}

fn command(w: i32, offset: i32) -> Rect {
    screen_rect(w, 40 + offset * 239, 1510, 228, 100)
}

fn difficulty_key(w: i32, offset: usize) -> Rect {
    screen_rect(w, 279 + offset as i32 * 237, 788, 220, 100)
}

fn cancel(w: i32) -> Rect {
    screen_rect(w, 522, 933, 220, 100)
}

impl App for View {
    fn draw(&self, canvas: &mut Canvas) {
        let width = canvas.bounds().w;
        header(canvas, "Sudoku", "Done");
        canvas.text(
            screen_rect(width, 92, 156, 1080, 57),
            &format!("{} puzzle", self.game.difficulty().name()),
            CAPTION,
        );
        self.draw_board(canvas, width);
        for digit in 1..=9 {
            canvas.button(digit_key(width, digit), &digit.to_string(), false, false);
        }
        let notice = if self.save_error {
            "Could not save this puzzle.".to_owned()
        } else if self.game.solved() {
            "Puzzle solved.".to_owned()
        } else if self.checked {
            match self.game.mistakes() {
                0 => "No mistakes so far.".to_owned(),
                1 => "One mistake is marked.".to_owned(),
                n => format!("{n} mistakes are marked."),
            }
        } else if self.notes {
            "Select an empty square to add or remove a pencil mark.".to_owned()
        } else {
            "Select a square, then a number.".to_owned()
        };
        canvas.text(
            screen_rect(width, 40, 1435, 1184, 52),
            &notice,
            CAPTION.center(),
        );
        for (offset, label) in [
            (0, "Notes"),
            (1, "Erase"),
            (2, "Undo"),
            (3, "Check"),
            (4, "New puzzle"),
        ] {
            canvas.button(
                command(width, offset),
                label,
                offset == 0 && self.notes,
                false,
            );
        }
        if self.new_game {
            canvas.popup(screen_rect(width, PUZZLE.x, PUZZLE.y, PUZZLE.w, PUZZLE.h));
            canvas.text(
                screen_rect(width, 279, 600, 706, 90),
                "Start a new puzzle?",
                TextStyle::new(50).bold().center(),
            );
            canvas.text(
                screen_rect(width, 279, 703, 706, 67),
                "This replaces your current puzzle.",
                CAPTION.center().ink(SECONDARY),
            );
            for (index, difficulty) in Difficulty::ALL.into_iter().enumerate() {
                canvas.button(difficulty_key(width, index), difficulty.name(), true, false);
            }
            canvas.button(cancel(width), "Cancel", false, false);
        }
    }

    fn input(&mut self, input: Input) -> Change {
        let Input::Tap(x, y) = input else {
            return Change::None;
        };
        let w = WIDTH as i32;
        if self.new_game {
            for (offset, difficulty) in Difficulty::ALL.into_iter().enumerate() {
                if difficulty_key(w, offset).contains(x, y) {
                    self.game = Sudoku::new(difficulty, seed());
                    self.selected = None;
                    self.notes = false;
                    self.checked = false;
                    self.new_game = false;
                    self.record();
                    return Change::Full;
                }
            }
            if cancel(w).contains(x, y)
                || !screen_rect(w, PUZZLE.x, PUZZLE.y, PUZZLE.w, PUZZLE.h).contains(x, y)
            {
                self.new_game = false;
                return Change::Full;
            }
            return Change::None;
        }
        if back_button(w).contains(x, y) {
            return Change::Exit;
        }
        if command(w, 4).contains(x, y) {
            self.new_game = true;
            return Change::Full;
        }
        if command(w, 0).contains(x, y) {
            self.notes = !self.notes;
            return Change::Partial;
        }
        if command(w, 3).contains(x, y) && !self.checked {
            self.checked = true;
            return Change::Partial;
        }
        if command(w, 2).contains(x, y) && self.game.undo() {
            self.record();
            return Change::Partial;
        }
        for row in 0..9 {
            for column in 0..9 {
                if tile(w, row, column).contains(x, y) && self.selected != Some((row, column)) {
                    self.selected = Some((row, column));
                    return Change::Partial;
                }
            }
        }
        if let Some((row, column)) = self.selected {
            let digit = if command(w, 1).contains(x, y) {
                Some(0)
            } else {
                (1..=9).find(|&i| digit_key(w, i).contains(x, y))
            };
            if digit.is_some_and(|digit| self.game.edit(row, column, digit, self.notes)) {
                self.record();
                return Change::Partial;
            }
        }
        Change::None
    }

    fn save(&mut self) -> io::Result<()> {
        if let Some(path) = &self.data {
            primitive_app::save(path, &serde_json::to_vec(&self.game)?)?;
        }
        Ok(())
    }
}
