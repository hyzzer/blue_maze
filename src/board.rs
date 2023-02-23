#[derive(Clone)]
#[derive(Debug)]
pub enum Boxes {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Board {
    pub boxes: Vec<Boxes>,
    pub player_coordinates: [usize; 2],
}

impl Board {
    fn initalize_boxes(size: usize) -> Vec<Boxes> {
        vec![Boxes::UP; size]
    }

    pub fn new(size: usize) -> Self {
        Self {
            boxes: Board::initalize_boxes(size),
            player_coordinates: [0, 0],
        }
    }
}