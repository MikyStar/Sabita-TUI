use core::fmt;

////////////////////////////////////////

const FULL_STAR: &str = "★";
const EMPTY_STAR: &str = "☆";

////////////////////////////////////////

#[derive(Debug)]
pub enum DIFFICULTY {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl fmt::Display for DIFFICULTY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nb_filled = match self {
            DIFFICULTY::One => 1,
            DIFFICULTY::Two => 2,
            DIFFICULTY::Three => 3,
            DIFFICULTY::Four => 4,
            DIFFICULTY::Five => 5,
        };

        let filled = FULL_STAR.repeat(nb_filled);

        let max_available = 5;
        let remaining_nb = max_available - nb_filled;
        let remaining_text = EMPTY_STAR.repeat(remaining_nb);

        write!(f, "{filled}{remaining_text}")
    }
}
