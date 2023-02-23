use rand::Rng;

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
        let mut boxes: Vec<Boxes> = Vec::with_capacity(size);   
        for _ in 0..size {
            boxes.push(get_random_box())
        }
        boxes
    }

    pub fn new(size: usize) -> Self {
        Self {
            boxes: Board::initalize_boxes(size),
            player_coordinates: [0, 0],
        }
    }
}