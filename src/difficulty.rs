use std::io;

pub enum Difficulty {
    Easy,
    Normal,
    Hard
}

impl Difficulty {
    pub const fn get_size(&self) -> usize {
        match *self {
            Difficulty::Easy => 16,
            Difficulty::Normal => 32,
            Difficulty::Hard => 64,
        }
    }
}

pub fn get_difficulty() -> Difficulty {
    let mut difficulty_input = String::new();

    println!("Choose difficulty : ");
    println!("1. EASY");
    println!("2. NORMAL");
    println!("3. HARD");
    
    io::stdin()
        .read_line(&mut difficulty_input)
        .expect("Failed to read user input");
    
    match difficulty_input.as_str() {
        "1" => Difficulty::Easy,
        "2" => Difficulty::Normal,
        "3" => Difficulty::Hard,
        _ => Difficulty::Easy,
    }
}