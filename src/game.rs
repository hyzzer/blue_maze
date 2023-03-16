use bevy::prelude::App;
use bevy::prelude::Commands;
use bevy::prelude::Plugin;
use bevy::prelude::Resource;
use bevy::prelude::StartupStage;

use crate::difficulty;
use crate::board;

#[derive(Resource)]
pub struct BoardSize {
    pub x: usize,
    pub y: usize,
}

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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, initalize_game);
    }
}

fn initalize_game(mut commands: Commands) {
    let game = Game::new(difficulty::Difficulty::Easy);
    let size = game.board.size;
    commands.insert_resource(game.board);
    commands.insert_resource( BoardSize {
            x: size,
            y: size,
    });

}