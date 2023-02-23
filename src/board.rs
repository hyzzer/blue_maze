use rand::Rng;
use std::fmt;

#[derive(Debug)]
pub enum Boxes {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Board {
    pub size: usize,
    pub boxes: Vec<Boxes>,
    pub player_coordinates: [usize; 2],
}

impl Board {
    fn initalize_boxes(size: usize) -> Vec<Boxes> {
        fn get_random_box() -> Boxes {
            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(0..4);
            match random_number {
                0 => Boxes::UP,
                1 => Boxes::DOWN,
                2 => Boxes::LEFT,
                3 => Boxes::RIGHT,
                _ => unreachable!()
            }
        }
        let mut boxes: Vec<Boxes> = Vec::with_capacity(size * size);   
        for _ in 0..size * size {
            boxes.push(get_random_box())
        }
        boxes
    }

    pub fn new(size: usize) -> Self {
        Self {
            size: size,
            boxes: Board::initalize_boxes(size),
            player_coordinates: [0, 0],
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_idx in 0..self.size {
            write!(f, "{:?}", &self.boxes[self.size*row_idx..self.size*row_idx+self.size])?;
            writeln!(f)?;
        }
        Ok(())
    }
}