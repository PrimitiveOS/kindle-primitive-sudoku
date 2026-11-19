//! Generate and play a Sudoku with exactly one solution.
use serde::{Deserialize, Serialize};

type Digits = [[u8; 9]; 9];
type Marks = [[u16; 9]; 9];
const DIGITS: u16 = 0b11_1111_1110;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub const ALL: [Self; 3] = [Self::Easy, Self::Medium, Self::Hard];

    pub fn name(self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
        }
    }

    fn clues(self) -> usize {
        match self {
            Self::Easy => 42,
            Self::Medium => 35,
            Self::Hard => 28,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Edit {
    row: usize,
    column: usize,
    value: u8,
    notes: u16,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Sudoku {
    difficulty: Difficulty,
    given: Digits,
    solution: Digits,
    entered: Digits,
    notes: Marks,
    undo: Vec<Edit>,
}

impl Sudoku {
    pub fn new(difficulty: Difficulty, seed: u64) -> Self {
        let mut rng = Rng(seed.max(1));
        let target = difficulty.clues();
        let (mut given, mut solution, mut clues) = (Digits::default(), Digits::default(), 82);
        for _ in 0..8 {
            let (puzzle, answer, count) = generate(target, &mut rng);
            if count < clues {
                (given, solution, clues) = (puzzle, answer, count);
            }
            if clues == target {
                break;
            }
        }
        Self {
            difficulty,
            given,
            solution,
            entered: given,
            notes: Marks::default(),
            undo: Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.solution.iter().flatten().all(|d| (1..=9).contains(d))
            && solution_count(self.solution) == 1
            && self
                .given
                .iter()
                .flatten()
                .zip(self.solution.iter().flatten())
                .all(|(given, solution)| *given == 0 || given == solution)
            && solution_count(self.given) == 1
            && self
                .entered
                .iter()
                .flatten()
                .zip(self.given.iter().flatten())
                .all(|(entered, given)| *entered <= 9 && (*given == 0 || entered == given))
            && self
                .notes
                .iter()
                .flatten()
                .all(|notes| notes & !DIGITS == 0)
            && self.undo.len() <= 512
            && self.undo.iter().all(|edit| {
                edit.row < 9
                    && edit.column < 9
                    && self.given[edit.row][edit.column] == 0
                    && edit.value <= 9
                    && edit.notes & !DIGITS == 0
            })
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn value(&self, row: usize, column: usize) -> u8 {
        self.entered[row][column]
    }

    pub fn given(&self, row: usize, column: usize) -> bool {
        self.given[row][column] != 0
    }

    pub fn notes(&self, row: usize, column: usize) -> u16 {
        self.notes[row][column]
    }

    pub fn mistake(&self, row: usize, column: usize) -> bool {
        let value = self.entered[row][column];
        value != 0 && value != self.solution[row][column]
    }

    pub fn mistakes(&self) -> usize {
        (0..9)
            .flat_map(|row| (0..9).map(move |column| (row, column)))
            .filter(|&(row, column)| self.mistake(row, column))
            .count()
    }

    pub fn solved(&self) -> bool {
        self.entered == self.solution
    }

    pub fn edit(&mut self, row: usize, column: usize, digit: u8, notes_mode: bool) -> bool {
        if row >= 9 || column >= 9 || digit > 9 || self.given(row, column) || self.solved() {
            return false;
        }
        let value = self.entered[row][column];
        let notes = self.notes[row][column];
        let (next_value, next_notes) = if notes_mode && digit != 0 {
            if value != 0 {
                return false;
            }
            (0, notes ^ (1 << digit))
        } else {
            (digit, 0)
        };
        if (value, notes) == (next_value, next_notes) {
            return false;
        }
        if self.undo.len() == 512 {
            self.undo.remove(0);
        }
        self.undo.push(Edit {
            row,
            column,
            value,
            notes,
        });
        self.entered[row][column] = next_value;
        self.notes[row][column] = next_notes;
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some(Edit {
            row,
            column,
            value,
            notes,
        }) = self.undo.pop()
        else {
            return false;
        };
        self.entered[row][column] = value;
        self.notes[row][column] = notes;
        true
    }
}

fn generate(target: usize, rng: &mut Rng) -> (Digits, Digits, usize) {
    let mut symbols = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    rng.shuffle(&mut symbols);
    let rows = permute_axis(rng);
    let columns = permute_axis(rng);
    let solution = std::array::from_fn(|r| {
        std::array::from_fn(|c| symbols[(rows[r] * 3 + rows[r] / 3 + columns[c]) % 9])
    });
    let mut given = solution;
    let mut order: [usize; 81] = std::array::from_fn(|index| index);
    rng.shuffle(&mut order);
    let mut clues = 81;
    for index in order {
        let (row, column) = (index / 9, index % 9);
        given[row][column] = 0;
        if solution_count(given) == 1 {
            clues -= 1;
            if clues == target {
                break;
            }
        } else {
            given[row][column] = solution[row][column];
        }
    }
    (given, solution, clues)
}

fn permute_axis(rng: &mut Rng) -> [usize; 9] {
    let mut groups = [0, 1, 2];
    rng.shuffle(&mut groups);
    let mut axis = [0; 9];
    for (index, group) in groups.into_iter().enumerate() {
        let mut inner = [0, 1, 2];
        rng.shuffle(&mut inner);
        for (position, row) in inner.into_iter().enumerate() {
            axis[index * 3 + position] = group * 3 + row;
        }
    }
    axis
}

fn solution_count(mut grid: Digits) -> u8 {
    let mut masks = [[0_u16; 9]; 3];
    for (row, values) in grid.iter().enumerate() {
        for (column, &digit) in values.iter().enumerate() {
            if digit == 0 {
                continue;
            }
            let b = (row / 3) * 3 + column / 3;
            let mask = 1 << digit;
            if (masks[0][row] | masks[1][column] | masks[2][b]) & mask != 0 {
                return 0;
            }
            masks[0][row] |= mask;
            masks[1][column] |= mask;
            masks[2][b] |= mask;
        }
    }
    solve(&mut grid, &mut masks, 2)
}

fn solve(grid: &mut Digits, masks: &mut [[u16; 9]; 3], limit: u8) -> u8 {
    let mut next = None;
    let mut fewest = 10;
    for (row, values) in grid.iter().enumerate() {
        for (column, &digit) in values.iter().enumerate() {
            if digit == 0 {
                let b = (row / 3) * 3 + column / 3;
                let options = DIGITS & !(masks[0][row] | masks[1][column] | masks[2][b]);
                let count = options.count_ones();
                if count == 0 {
                    return 0;
                }
                if count < fewest {
                    next = Some((row, column, b, options));
                    fewest = count;
                }
            }
        }
    }
    let Some((row, column, b, mut options)) = next else {
        return 1;
    };
    let mut count = 0;
    while options != 0 && count < limit {
        let digit = options.trailing_zeros() as u8;
        let bit = 1 << digit;
        options ^= bit;
        grid[row][column] = digit;
        masks[0][row] ^= bit;
        masks[1][column] ^= bit;
        masks[2][b] ^= bit;
        count += solve(grid, masks, limit - count);
        masks[0][row] ^= bit;
        masks[1][column] ^= bit;
        masks[2][b] ^= bit;
        grid[row][column] = 0;
    }
    count
}

struct Rng(u64);

impl Rng {
    fn shuffle<T>(&mut self, values: &mut [T]) {
        for index in (1..values.len()).rev() {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            values.swap(index, (self.0 % (index + 1) as u64) as usize);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_puzzles_have_one_solution_and_distinct_difficulty_targets() {
        for seed in 1..=16 {
            for difficulty in Difficulty::ALL {
                let game = Sudoku::new(difficulty, seed);
                assert!(game.is_valid());
                assert_eq!(solution_count(game.given), 1, "{difficulty:?} seed {seed}");
                assert_eq!(
                    game.given.iter().flatten().filter(|&&d| d != 0).count(),
                    difficulty.clues()
                );
            }
        }
    }

    #[test]
    fn editable_cells_check_and_undo_digits_and_pencil_marks() {
        let mut game = Sudoku::new(Difficulty::Easy, 12);
        let (r, c) = (0..9)
            .flat_map(|r| (0..9).map(move |c| (r, c)))
            .find(|&(r, c)| !game.given(r, c))
            .unwrap();
        assert!(game.edit(r, c, 2, true));
        assert_eq!(game.notes(r, c), 1 << 2);
        let wrong = game.solution[r][c] % 9 + 1;
        assert!(game.edit(r, c, wrong, false));
        assert_eq!(game.notes(r, c), 0);
        assert_eq!(game.mistakes(), 1);
        assert!(game.undo());
        assert_eq!(game.notes(r, c), 1 << 2);
        assert!(game.undo());
        assert_eq!(game.notes(r, c), 0);
        assert!(!game.undo());
    }

    #[test]
    fn every_given_is_locked_and_correct_entries_solve_the_puzzle() {
        let mut game = Sudoku::new(Difficulty::Hard, 23);
        for row in 0..9 {
            for column in 0..9 {
                let digit = game.solution[row][column];
                if game.given(row, column) {
                    assert!(!game.edit(row, column, 0, false));
                } else {
                    assert!(game.edit(row, column, digit, false));
                }
            }
        }
        assert!(game.solved());
        assert!(game.undo());
        assert!(!game.solved());
    }
}
