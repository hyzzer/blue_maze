use bevy::prelude::*;
use board::WallStatus;
use player::PlayerPlugin;
use tilemap::TilemapPlugin;

mod game;
mod difficulty;
mod board;
mod player;
mod components;
mod assets;
mod tilemap;

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct BoardSize {
    pub x: usize,
    pub y: usize,
}

#[derive(Resource)]
pub struct Walls {
    horizontal_walls: Vec<WallStatus>,
    vertical_walls: Vec<WallStatus>,
}

#[derive(Resource)]
pub struct GameTextures {
    player: Handle<Image>,
    tile_top: Handle<Image>,
    tile_bottom: Handle<Image>,
    tile_wall: Handle<Image>,
    tile_road: Handle<Image>,
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    let (window_width, window_height) = (window.width(), window.height());

    let win_size = WinSize {
        w: window_width,
        h: window_height
    };
    commands.insert_resource(win_size);

    let game_textures = GameTextures {
        player: asset_server.load(assets::PlayerSprite::Golem01.get_path()),
        tile_top: asset_server.load(assets::TILE_TOP),
        tile_bottom: asset_server.load(assets::TILE_BOTTOM),
        tile_wall: asset_server.load(assets::TILE_WALL),
        tile_road: asset_server.load(assets::TILE_ROAD),
    };
    commands.insert_resource(game_textures);
}

fn main() {
    let game = game::Game::new(difficulty::Difficulty::Easy);
    println!("{}", game.board);
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.)))
        .insert_resource(BoardSize {
            x: game.board.size,
            y: game.board.size,
        })
        .insert_resource(Walls {
            horizontal_walls: game.board.horizontal_walls,
            vertical_walls: game.board.vertical_walls,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Blue Maze".to_string(),
                // mode: WindowMode::Fullscreen,
                width: 1500.0,
                height: 700.0,
                decorations: true,
                resizable: false,
              ..default()
            },
            ..default()
          }))
        .add_plugin(PlayerPlugin)
        .add_plugin(TilemapPlugin)
        .add_startup_system(setup_system)
        .run();
}
