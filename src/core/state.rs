use std::{time::Instant, usize};

use sabita::core::{
    constants::{LENGTH_DIMENSION, TO_BE_SOLVED},
    grid::{BoxLocation, Grid},
    validation::validate,
};

use crate::core::difficulty::{DIFFICULTY, MAX_DIFFICULTY_INDEX};

////////////////////////////////////////

pub const LENGTH_USIZE: usize = LENGTH_DIMENSION as usize;
pub const BASE_DIFFICULTY: DIFFICULTY = DIFFICULTY::One;

////////////////////////////////////////

pub struct State {
    /////////////////
    // Public
    pub grid_to_solve: Grid,
    pub original_grid: Grid,
    pub full_grid: Grid,

    pub cursor_row: usize,
    pub cursor_col: usize,

    pub original_nb_missing_values: u8,
    pub remaining_nb_missing_values: u8,

    pub is_solved: Option<bool>,

    pub start: Instant,
    pub solved_at: Option<Instant>,

    pub difficulty: DIFFICULTY,

    pub streak: u8,

    pub is_zen_mode: bool,

    /////////////////
    // Private
    memoized_missing_box_locations: Vec<BoxLocation>,
}

/////////////////

impl State {
    pub fn new(difficulty: Option<DIFFICULTY>) -> State {
        let difficulty = difficulty.unwrap_or(BASE_DIFFICULTY);
        let nb_missing_cells: u8 = difficulty.get_missing_cell_nb();

        let full_grid = Grid::generate(None);

        let mut grid = full_grid.clone();
        grid.remove_random_values(nb_missing_cells);

        let memoized_missing_box_locations = grid.locate_missing_box();
        let BoxLocation { line, column, .. } = memoized_missing_box_locations[0];

        State {
            grid_to_solve: grid.clone(),
            original_grid: grid.clone(),
            full_grid,

            cursor_row: line,
            cursor_col: column,

            original_nb_missing_values: nb_missing_cells,
            remaining_nb_missing_values: nb_missing_cells,

            is_solved: None,

            start: Instant::now(),
            solved_at: None,

            difficulty,

            streak: 0,

            is_zen_mode: false,

            memoized_missing_box_locations,
        }
    }

    /////////////////

    // Moving

    pub fn move_cell_left(&mut self) {
        for col_index in (0..self.cursor_col).rev() {
            let cell = self.original_grid.values[self.cursor_row][col_index];

            if cell == TO_BE_SOLVED {
                self.cursor_col = col_index;
                break;
            }
        }
    }

    pub fn move_cell_right(&mut self) {
        let next_col = self.cursor_col + 1;

        if next_col >= LENGTH_USIZE {
            return;
        }

        for col_index in next_col..LENGTH_USIZE {
            let cell = self.original_grid.values[self.cursor_row][col_index];

            if cell == 0 {
                self.cursor_col = col_index;
                break;
            }
        }
    }

    pub fn move_cell_top(&mut self) {
        for row_index in (0..self.cursor_row).rev() {
            let cell = self.original_grid.values[row_index][self.cursor_col];

            if cell == 0 {
                self.cursor_row = row_index;
                break;
            }
        }
    }

    pub fn move_cell_bottom(&mut self) {
        let next_row = self.cursor_row + 1;

        if next_row >= LENGTH_USIZE {
            return;
        }

        for row_index in next_row..LENGTH_USIZE {
            let cell = self.original_grid.values[row_index][self.cursor_col];

            if cell == 0 {
                self.cursor_row = row_index;
                break;
            }
        }
    }

    pub fn move_next_cell(&mut self) {
        let missing_cells = self.memoized_missing_box_locations.clone();

        for (index, cell) in missing_cells.iter().enumerate() {
            let BoxLocation { line, column, .. } = cell;

            let is_current_cell = *line == self.cursor_row && *column == self.cursor_col;

            if !is_current_cell {
                continue;
            }

            let is_next_available = index + 1 < missing_cells.len();

            if is_next_available {
                let BoxLocation {
                    line: next_row,
                    column: next_col,
                    ..
                } = missing_cells[index + 1];

                self.cursor_row = next_row;
                self.cursor_col = next_col;

                return;
            }
        }

        // If not found before, go back to first available cell
        let BoxLocation { line, column, .. } = missing_cells[0];

        self.cursor_row = line;
        self.cursor_col = column;
    }

    pub fn move_previous_cell(&mut self) {
        let missing_cells = self.memoized_missing_box_locations.clone();

        for (index, cell) in missing_cells.iter().enumerate() {
            let BoxLocation { line, column, .. } = cell;

            let is_current_cell = *line == self.cursor_row && *column == self.cursor_col;

            if !is_current_cell {
                continue;
            }

            if let Some(new_index) = index.checked_sub(1) {
                let BoxLocation {
                    line: next_row,
                    column: next_col,
                    ..
                } = missing_cells[new_index];

                self.cursor_row = next_row;
                self.cursor_col = next_col;

                return;
            }
        }

        // If not found before, go back to first available cell
        let BoxLocation { line, column, .. } = missing_cells[missing_cells.len() - 1];

        self.cursor_row = line;
        self.cursor_col = column;
    }

    // Filling

    pub fn set_number(&mut self, num: u8) {
        if num >= 1 && num <= LENGTH_DIMENSION {
            let previous_value = self.grid_to_solve.values[self.cursor_row][self.cursor_col];
            self.grid_to_solve.values[self.cursor_row][self.cursor_col] = num;

            if previous_value == TO_BE_SOLVED {
                self.remaining_nb_missing_values -= 1;
            }

            if self.remaining_nb_missing_values == 0 {
                match validate(&self.grid_to_solve.values) {
                    Ok(_) => {
                        self.is_solved = Some(true);
                        self.solved_at = Some(Instant::now());
                    }
                    Err(_) => self.is_solved = Some(false),
                }
            }
        }
    }

    pub fn clear_cell(&mut self) {
        let original_value = self.original_grid.values[self.cursor_row][self.cursor_col];
        self.grid_to_solve.values[self.cursor_row][self.cursor_col] = original_value;

        self.remaining_nb_missing_values += 1;
        self.is_solved = None;
    }

    // App controls

    pub fn reset(&mut self) {
        let BoxLocation { line, column, .. } = self.memoized_missing_box_locations[0];

        self.grid_to_solve = self.original_grid.clone();

        self.cursor_row = line;
        self.cursor_col = column;

        self.remaining_nb_missing_values = self.original_nb_missing_values;

        self.is_solved = None;
    }

    pub fn increase_difficulty(&mut self) {
        let current_difficulty_index: usize = self.difficulty.into();

        if current_difficulty_index < MAX_DIFFICULTY_INDEX {
            let next_difficulty = DIFFICULTY::try_from(current_difficulty_index + 1).unwrap();
            *self = State::new(Some(next_difficulty));
        }
    }

    pub fn decrease_difficulty(&mut self) {
        let current_difficulty_index: usize = self.difficulty.into();

        if current_difficulty_index > 0 {
            let next_difficulty = DIFFICULTY::try_from(current_difficulty_index - 1).unwrap();
            *self = State::new(Some(next_difficulty));
        }
    }

    pub fn new_from_same_difficulty(&mut self) {
        *self = State::new(Some(self.difficulty));
    }

    pub fn toggle_zen_mode(&mut self) {
        self.is_zen_mode = !self.is_zen_mode;
    }

    pub fn solve(&mut self) {
        if self.is_solved.is_some() {
            return;
        }

        self.grid_to_solve = self.full_grid.clone();
        self.is_solved = Some(true);
        self.solved_at = Some(Instant::now());
    }
}
