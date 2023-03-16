use bevy::prelude::Resource;
use rand::Rng;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WallStatus {
    OPEN,
    CLOSED,
    EDGE,
}

#[derive(Resource)]
pub struct Board {
    pub size: usize,
    pub player_coordinates: [usize; 2],
    pub vertical_walls: Vec<WallStatus>,
    pub horizontal_walls: Vec<WallStatus>,
}

impl Board {
    pub fn is_wall_open(&self, box_idx: usize, direction: &Direction) -> WallStatus {
        if box_idx == self.size * self.size {
            if *direction == Direction::RIGHT {
                return WallStatus::OPEN;
            } else {
                return WallStatus::CLOSED;
            }
        }
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
                if box_idx >= self.size*(self.size - 1) {
                    WallStatus::EDGE
                } else {
                    self.horizontal_walls[box_idx]
                }
            },             
            Direction::LEFT => {
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
        for _ in 0..self.size * (self.size - 1) {
            self.horizontal_walls.push(WallStatus::CLOSED);
            self.vertical_walls.push(WallStatus::CLOSED);
        }
    }

    fn kruskal_algorithm(&mut self) {
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
        
        let mut step = 0;
        let mut sequences: Vec<usize> = Vec::with_capacity(self.size * self.size);
        for sequence_idx in 0..self.size * self.size {
            sequences.push(sequence_idx);
        }

        while step != self.size*self.size - 1 {
            let random_box = get_random_box(self.size * self.size);
            let random_direction = get_random_direction();
            let old_sequence = sequences[random_box];
            if self.is_wall_open(random_box, &random_direction) == WallStatus::CLOSED {
                let wall_idx: usize;
                let new_sequence: usize;
                let row_idx = ((random_box/self.size) as f32).floor();
                match random_direction {
                    Direction::UP => {
                        new_sequence = sequences[random_box - self.size];
                        wall_idx = random_box - self.size;
                    },
                    Direction::DOWN => {
                        new_sequence = sequences[random_box + self.size];
                        wall_idx = random_box;
                    },
                    Direction::LEFT => {
                        new_sequence = sequences[random_box - 1];
                        wall_idx = random_box - row_idx as usize - 1;
                    },
                    Direction::RIGHT => {
                        new_sequence = sequences[random_box + 1];
                        wall_idx = random_box - row_idx as usize;
                    },
                };
                if old_sequence != new_sequence {
                    if random_direction == Direction::UP || random_direction == Direction::DOWN {
                        self.horizontal_walls[wall_idx] = WallStatus::OPEN;
                    } else {
                        self.vertical_walls[wall_idx] = WallStatus::OPEN;
                    }
    
                    for box_idx in 0..self.size * self.size {
                        if sequences[box_idx] == old_sequence {
                            sequences[box_idx] = new_sequence;
                        }
                    }
                    step += 1;
                }
            }
        }
    }

    pub fn new(size: usize) -> Self {
        let mut new_board = Self {
            size: size,
            player_coordinates: [0, 0],
            horizontal_walls: Vec::with_capacity(size * (size - 1)),
            vertical_walls: Vec::with_capacity(size * (size - 1)),
        };
        new_board.initalize_walls();
        new_board.kruskal_algorithm();
        new_board
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Vertical
        for row_idx in 0..self.size {
            write!(f, "{:?}", &self.vertical_walls[(self.size - 1)*row_idx..(self.size - 1)*row_idx + self.size - 1])?;
            writeln!(f)?;
        }
        writeln!(f)?;
        // Horizontal
        for row_idx in 0..self.size - 1 {
            write!(f, "{:?}", &self.horizontal_walls[(self.size)*row_idx..self.size*row_idx + self.size])?;
            writeln!(f)?;
        }   
        Ok(())
    }
}