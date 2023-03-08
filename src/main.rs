use bevy::prelude::*;
use player::PlayerPlugin;

mod game;
mod difficulty;
mod board;
mod player;

enum PlayerSprite {
    Golem01,
    Golem02,
    Golem03,
}

impl PlayerSprite {
    fn get_path(&self) -> &str {
        match *self {
            PlayerSprite::Golem01 => "golem_01.png",
            PlayerSprite::Golem02 => "golem_02.png",
            PlayerSprite::Golem03 => "golem_03.png",
        }
    }
}

const PLAYER_SIZE: (f32, f32) = (410., 560.);
const SPRITE_SCALE: f32 = 0.15;

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    player: Handle<Image>,
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    let (window_width, window_height) = (window.width(), window.height());

    let win_size = WinSize{
        w: window_width,
        h: window_height
    }; 
    commands.insert_resource(win_size);

    let game_textures = GameTextures {
        player: asset_server.load(PlayerSprite::Golem01.get_path()),
    };
    commands.insert_resource(game_textures);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Blue Maze".to_string(),
                // mode: WindowMode::Fullscreen,
                // width: 900.0,
                // height: 700.0,
                fit_canvas_to_parent: true,
                decorations: true,
              ..default()
            },
            ..default()
          }))
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .run();

    let game = game::Game::new(difficulty::Difficulty::Easy);
}
