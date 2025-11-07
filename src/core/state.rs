use std::usize;

use sabita::core::{constants::LENGTH_DIMENSION, grid::Grid, validation::validate};

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

        State {
            grid_to_solve: grid.clone(),
            original_grid: grid.clone(),

            cursor_row: 0,
            cursor_col: 0,

            original_nb_missing_values: BASE_MISSING_VALUES,
            remaining_nb_missing_values: BASE_MISSING_VALUES,
        }
    }

    /////////////////
    // Public

    pub fn move_cell_left(&mut self) {
        for col_index in (0..self.cursor_col).rev() {
            let cell = self.original_grid.values[self.cursor_row][col_index];

            if cell == 0 {
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

    pub fn set_number(&mut self, num: u8) {
        if num >= 1 && num <= LENGTH_DIMENSION {
            self.grid_to_solve.values[self.cursor_row][self.cursor_col] = num;

            self.remaining_nb_missing_values -= 1;
        }
    }

    pub fn clear_cell(&mut self) {
        let original_value = self.original_grid.values[self.cursor_row][self.cursor_col];
        self.grid_to_solve.values[self.cursor_row][self.cursor_col] = original_value;

        self.remaining_nb_missing_values += 1;
    }

    fn reset_state(&mut self) {
        self.grid_to_solve = self.original_grid.clone();

        self.cursor_row = 0;
        self.cursor_col = 0;

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
