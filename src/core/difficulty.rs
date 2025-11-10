use core::fmt;

use sabita::core::constants::{LENGTH_DIMENSION, MINIMUM_PROVIDED};

////////////////////////////////////////

const FULL_STAR: &str = "★";
const EMPTY_STAR: &str = "☆";
pub const MAX_DIFFICULTY_INDEX: usize = 4;

////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
#[repr(usize)]
pub enum DIFFICULTY {
    One,
    Two,
    Three,
    Four,
    Five,
}

////////////////////////////////////////

impl fmt::Display for DIFFICULTY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nb_filled = self.get_index() + 1;
        let filled = FULL_STAR.repeat(nb_filled);

        let remaining_nb = (MAX_DIFFICULTY_INDEX + 1) - nb_filled;
        let remaining_text = EMPTY_STAR.repeat(remaining_nb);

        write!(f, "{filled}{remaining_text}")
    }
}

impl From<DIFFICULTY> for usize {
    fn from(val: DIFFICULTY) -> Self {
        val as usize
    }
}

impl From<usize> for DIFFICULTY {
    fn from(index: usize) -> Self {
        match index {
            0 => DIFFICULTY::One,
            1 => DIFFICULTY::Two,
            2 => DIFFICULTY::Three,
            3 => DIFFICULTY::Four,
            4 => DIFFICULTY::Five,
            _ => DIFFICULTY::Five,
        }
    }
}

impl From<u8> for DIFFICULTY {
    fn from(index: u8) -> Self {
        match index {
            0 => DIFFICULTY::One,
            1 => DIFFICULTY::Two,
            2 => DIFFICULTY::Three,
            3 => DIFFICULTY::Four,
            4 => DIFFICULTY::Five,
            _ => DIFFICULTY::Five,
        }
    }
}

impl DIFFICULTY {
    pub fn get_index(&self) -> usize {
        *self as usize
    }

    pub fn get_missing_cell_nb(&self) -> u8 {
        let total_cells = LENGTH_DIMENSION * LENGTH_DIMENSION;

        match self {
            DIFFICULTY::One => 10,
            DIFFICULTY::Two => 20,
            DIFFICULTY::Three => 30,
            DIFFICULTY::Four => 45,
            DIFFICULTY::Five => total_cells - MINIMUM_PROVIDED,
        }
    }
}
