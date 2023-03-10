use crate::difficulty;
use crate::board;

pub const SPEED: f32 = 50.;
pub struct Game {
    difficulty: difficulty::Difficulty,
    pub board: board::Board,
}

impl Game {
    pub fn new(difficulty: difficulty::Difficulty) -> Self {
        let size = difficulty.get_size();
        Self {
            difficulty: difficulty,
            board: board::Board::new(size),
        }
    }
}