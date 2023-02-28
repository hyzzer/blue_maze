use rand::Rng;
use std::fmt;

#[derive(Debug)]
pub enum Boxes {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
enum WallStatus {
    OPEN,
    CLOSED,
    EDGE,
}

pub struct Board {
    pub size: usize,
    pub boxes: Vec<Boxes>,
    pub player_coordinates: [usize; 2],
    vertical_walls: Vec<WallStatus>,
    horizontal_walls: Vec<WallStatus>,
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

    fn is_wall_open(&self, box_idx: usize, direction: Direction) -> WallStatus {
        let row_idx = (((box_idx + 1)/self.size) as f32).floor();
        match direction {
            Direction::UP => {
                if box_idx < self.size {
                    WallStatus::EDGE
                } else {
                    self.horizontal_walls[box_idx - self.size]
                }
            }, 
            Direction::DOWN => {
                if box_idx > self.size*(self.size - 1) {
                    WallStatus::EDGE
                } else {
                    self.horizontal_walls[box_idx]
                }
            },             Direction::LEFT => {
                if box_idx % self.size == 0 {
                    WallStatus::EDGE
                } else {
                    self.vertical_walls[box_idx - row_idx as usize - 1]
                }
            },
            Direction::RIGHT => {
                if (box_idx + 1) % self.size == 0 {
                    WallStatus::EDGE
                } else {
                    self.vertical_walls[box_idx - row_idx as usize]
                }
            },        
        }

    }

    fn initalize_walls(&mut self) {
        for _ in 0..self.size {
            self.horizontal_walls.push(WallStatus::CLOSED);
            self.vertical_walls.push(WallStatus::CLOSED);
        }
    }

    fn kruskal_algorithm(&self, size: usize) {
        fn get_random_box(size: usize) -> usize {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..size)
        }
        
        fn get_random_direction() -> Direction {
            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(0..4);
            match random_number {
                0 => Direction::UP,
                1 => Direction::DOWN,
                2 => Direction::LEFT,
                3 => Direction::RIGHT,
                _ => unreachable!()
            }
        }
        
        let step = 0;
        let sequence_numbers: Vec<usize> = Vec::with_capacity(size * size);
        // TODO : give to each element its index as value
        let walls: Vec<WallStatus> = Vec::with_capacity(2*size*size-2*size);
        while step != size*size-1 {
            let random_box = get_random_box(size * size);
            let random_direction = get_random_direction();
            if self.is_wall_open(random_box, random_direction) == WallStatus::CLOSED {

                // TODO : change all the boxes with the right sequenceNumber (random_box)
                // So a for loop with verification on the old sequenceNumber is needed

                // TODO : change the status of the wall (CLOSED -> OPEN) (on horizontal and vertical vectors)
                // Maybe create a function to do this (it's borring, you have to say if its LEFT then, if its RIGHT then ...)
            }
        }
    }

    pub fn new(size: usize) -> Self {
        let mut new_board = Self {
            size: size,
            boxes: Board::initalize_boxes(size),
            player_coordinates: [0, 0],
            horizontal_walls: Vec::with_capacity(size),
            vertical_walls: Vec::with_capacity(size),
        };
        new_board.initalize_walls();
        new_board.kruskal_algorithm(size);
        new_board
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