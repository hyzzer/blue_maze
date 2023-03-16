use bevy::prelude::*;
use game::GamePlugin;
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
pub struct GameTextures {
    player: Handle<Image>,
    tile_top: Handle<Image>,
    tile_bottom: Handle<Image>,
    tile_road: Handle<Image>,
    tile_left_01: Handle<Image>,
    tile_right_01: Handle<Image>,
    tile_background: Handle<Image>,
    tile_top_left_corner: Handle<Image>,
    tile_top_right_corner_02: Handle<Image>,
    tile_bottom_left_corner_02: Handle<Image>,
    tile_bottom_right_corner: Handle<Image>,
    tile_enter_top_01: Handle<Image>,
    tile_enter_top_02: Handle<Image>,
    tile_exit_bottom: Handle<Image>,
    door_closed: Handle<Image>,
    chest_closed: Handle<Image>,
    tile_exit_below: Handle<Image>,
    tile_outline_top: Handle<Image>,
    tile_outline_top_left: Handle<Image>,
    tile_outline_top_right: Handle<Image>,
    tile_outline_bottom: Handle<Image>,
    tile_outline_bottom_left: Handle<Image>,
    tile_outline_bottom_right: Handle<Image>,
    tile_outline_left: Handle<Image>,
    tile_outline_right: Handle<Image>,
    tile_road_enter: Handle<Image>,
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
        tile_road: asset_server.load(assets::TILE_ROAD),
        tile_left_01: asset_server.load(assets::TILE_LEFT_01),
        tile_right_01: asset_server.load(assets::TILE_RIGHT_01),
        tile_background: asset_server.load(assets::TILE_BACKGROUND),
        tile_top_left_corner: asset_server.load(assets::TILE_TOP_LEFT_CORNER),
        tile_top_right_corner_02: asset_server.load(assets::TILE_TOP_RIGHT_CORNER_02),
        tile_bottom_left_corner_02: asset_server.load(assets::TILE_BOTTOM_LEFT_CORNER_02),
        tile_bottom_right_corner: asset_server.load(assets::TILE_BOTTOM_RIGHT_CORNER),
        tile_enter_top_01: asset_server.load(assets::TILE_ENTER_TOP_01),
        tile_enter_top_02: asset_server.load(assets::TILE_ENTER_TOP_02),
        tile_exit_bottom: asset_server.load(assets::TILE_EXIT_BOTTOM),
        chest_closed: asset_server.load(assets::CHEST_CLOSED),
        door_closed: asset_server.load(assets::DOOR_CLOSED),
        tile_exit_below: asset_server.load(assets::TILE_EXIT_BELOW),
        tile_outline_top: asset_server.load(assets::TILE_OUTLINE_TOP),
        tile_outline_top_left: asset_server.load(assets::TILE_OUTLINE_TOP_LEFT),
        tile_outline_top_right: asset_server.load(assets::TILE_OUTLINE_TOP_RIGHT),
        tile_outline_bottom: asset_server.load(assets::TILE_OUTLINE_BOTTOM),
        tile_outline_bottom_left: asset_server.load(assets::TILE_OUTLINE_BOTTOM_LEFT),
        tile_outline_bottom_right: asset_server.load(assets::TILE_OUTLINE_BOTTOM_RIGHT),
        tile_outline_left: asset_server.load(assets::TILE_OUTLINE_LEFT),
        tile_outline_right: asset_server.load(assets::TILE_OUTLINE_RIGHT),
        tile_road_enter: asset_server.load(assets::TILE_ROAD_ENTER),

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
                width: 1500.0,
                height: 700.0,
                decorations: true,
                // resizable: false,
              ..default()
            },
            ..default()
          }))
        .add_startup_system(setup_system)
        .add_plugin(GamePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TilemapPlugin)
        .run();
}
