use std::io;

#[derive(PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Error,
}

impl Difficulty {
    pub fn get_size(&self) -> usize {
        match *self {
            Difficulty::Easy => 16,
            Difficulty::Normal => 32,
            Difficulty::Hard => 64,
            Difficulty::Error => 0,
        }
    }
}

pub fn get_difficulty() -> Difficulty {
    
    let mut result = Difficulty::Error;
    
    while result == Difficulty::Error {
        let mut difficulty_input = String::new();
        println!("\nChoose difficulty : ");
        println!("1. EASY");
        println!("2. NORMAL");
        println!("3. HARD\n");
        
        io::stdin()
            .read_line(&mut difficulty_input)
            .expect("Failed to read user input");

        println!("{}", difficulty_input);
        
        result = match difficulty_input.trim() {
            "1" => Difficulty::Easy,
            "2" => Difficulty::Normal,
            "3" => Difficulty::Hard,
            _ => Difficulty::Error,
        };
    }
    result
}