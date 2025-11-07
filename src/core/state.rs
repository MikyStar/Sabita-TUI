use std::usize;

use sabita::core::{
    constants::{LENGTH_DIMENSION, TO_BE_SOLVED},
    grid::{BoxLocation, Grid},
    validation::validate,
};

////////////////////////////////////////

pub const LENGTH_USIZE: usize = LENGTH_DIMENSION as usize;
pub const BASE_MISSING_VALUES: u8 = 30;

////////////////////////////////////////

pub struct State {
    pub grid_to_solve: Grid,
    pub original_grid: Grid,

    pub cursor_row: usize,
    pub cursor_col: usize,

    pub original_nb_missing_values: u8,
    pub remaining_nb_missing_values: u8,
}

/////////////////

impl State {
    pub fn new() -> State {
        let grid = Grid::generate(Some(BASE_MISSING_VALUES));

        let BoxLocation { line, column, .. } = grid.locate_missing_box()[0];
        State {
            grid_to_solve: grid.clone(),
            original_grid: grid.clone(),

            cursor_row: line,
            cursor_col: column,

            original_nb_missing_values: BASE_MISSING_VALUES,
            remaining_nb_missing_values: BASE_MISSING_VALUES,
        }
    }

    /////////////////
    // Public

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
        let missing_cells = self.original_grid.locate_missing_box();

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
    pub fn set_number(&mut self, num: u8) {
        if num >= 1 && num <= LENGTH_DIMENSION {
            let previous_value = self.grid_to_solve.values[self.cursor_row][self.cursor_col];
            self.grid_to_solve.values[self.cursor_row][self.cursor_col] = num;

            if previous_value == TO_BE_SOLVED {
                self.remaining_nb_missing_values -= 1;

                if self.remaining_nb_missing_values == 0 {
                    match validate(&self.grid_to_solve.values) {
                        Ok(_) => self.is_solved = Some(true),
                        Err(_) => self.is_solved = Some(false),
                    }
                }
            }
        }
    }

    pub fn clear_cell(&mut self) {
        let original_value = self.original_grid.values[self.cursor_row][self.cursor_col];
        self.grid_to_solve.values[self.cursor_row][self.cursor_col] = original_value;

        self.remaining_nb_missing_values += 1;
    }

    fn reset_state(&mut self) {
        let base_grid = self.original_grid.clone();

        let BoxLocation { line, column, .. } = base_grid.locate_missing_box()[0];

        self.grid_to_solve = base_grid;

        self.cursor_row = line;
        self.cursor_col = column;

        self.remaining_nb_missing_values = self.original_nb_missing_values;
    }

    /////////////////
    // Private

    pub fn move_cursor(&mut self, row_action: i32, column_action: i32) {
        let new_row = (self.cursor_row as i32 + row_action).clamp(0, 8) as usize;
        let new_col = (self.cursor_col as i32 + column_action).clamp(0, 8) as usize;

        self.cursor_row = new_row;
        self.cursor_col = new_col;
    }
}
