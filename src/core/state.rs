use sabita::core::constants::LENGTH_DIMENSION;

////////////////////////////////////////

pub const LENGTH_USIZE: usize = LENGTH_DIMENSION as usize;

////////////////////////////////////////

pub struct State {
    pub grid: [[Option<u8>; LENGTH_USIZE]; LENGTH_USIZE],

    pub cursor_row: usize,
    pub cursor_col: usize,
}

/////////////////

impl State {
    pub fn new() -> State {
        State {
            grid: [[None; LENGTH_USIZE]; LENGTH_USIZE],

            cursor_row: 0,
            cursor_col: 0,
        }
    }

    /////////////////

    pub fn move_cursor(&mut self, dr: i32, dc: i32) {
        let new_row = (self.cursor_row as i32 + dr).clamp(0, 8) as usize;
        let new_col = (self.cursor_col as i32 + dc).clamp(0, 8) as usize;

        self.cursor_row = new_row;
        self.cursor_col = new_col;
    }

    pub fn set_number(&mut self, num: u8) {
        if num >= 1 && num <= LENGTH_DIMENSION {
            self.grid[self.cursor_row][self.cursor_col] = Some(num);
        }
    }

    pub fn clear_cell(&mut self) {
        self.grid[self.cursor_row][self.cursor_col] = None;
    }
}
