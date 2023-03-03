mod game;
mod difficulty;
mod board;

fn main() {
    let game = game::Game::new(difficulty::Difficulty::Easy);
}
