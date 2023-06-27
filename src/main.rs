mod assets;
mod components;
mod resources;
mod systems;
mod utils;

use resources::animations::PlayerTextureAtlasConfig;
use assets::player::PlayerSprite;
use bevy::{prelude::*};
use resources::game::GamePlugin;
use systems::player::PlayerPlugin;
use systems::tilemap::TilemapPlugin;
use winit::{self, event_loop};

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    tile_road: Handle<Image>,
    tile_background: Handle<Image>,
    tile_top: Handle<Image>,
    tile_bottom: Handle<Image>,
    tile_left: Handle<Image>,
    tile_right: Handle<Image>,
    tile_top_left_corner: Handle<Image>,
    tile_top_right_corner: Handle<Image>,
    tile_bottom_left_corner: Handle<Image>,
    tile_bottom_right_corner: Handle<Image>,
    tile_enter_top_01: Handle<Image>,
    tile_enter_top_02: Handle<Image>,
    tile_exit_bottom_01: Handle<Image>,
    tile_exit_bottom_02: Handle<Image>,
    tile_outline_top: Handle<Image>,
    tile_outline_top_left: Handle<Image>,
    tile_outline_top_right: Handle<Image>,
    tile_outline_bottom: Handle<Image>,
    tile_outline_bottom_left: Handle<Image>,
    tile_outline_bottom_right: Handle<Image>,
    tile_outline_left: Handle<Image>,
    tile_outline_right: Handle<Image>,
    tile_road_enter: Handle<Image>,
    door_closed: Handle<Image>,
    door_open: Handle<Image>,
    chest_locked: Handle<Image>,
    chest_unlocked: Handle<Image>,
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
        chest_locked: asset_server.load(assets::objects::CHEST_LOCKED),
        chest_unlocked: asset_server.load(assets::objects::CHEST_UNLOCKED),
        door_closed: asset_server.load(assets::objects::DOOR_CLOSED),
        door_open: asset_server.load(assets::objects::DOOR_OPEN),
        tile_top: asset_server.load(assets::tilemap::TILE_TOP),
        tile_bottom: asset_server.load(assets::tilemap::TILE_BOTTOM),
        tile_road: asset_server.load(assets::tilemap::TILE_ROAD),
        tile_left: asset_server.load(assets::tilemap::TILE_LEFT),
        tile_right: asset_server.load(assets::tilemap::TILE_RIGHT),
        tile_background: asset_server.load(assets::tilemap::TILE_BACKGROUND),
        tile_top_left_corner: asset_server.load(assets::tilemap::TILE_TOP_LEFT_CORNER),
        tile_top_right_corner: asset_server.load(assets::tilemap::TILE_TOP_RIGHT_CORNER),
        tile_bottom_left_corner: asset_server.load(assets::tilemap::TILE_BOTTOM_LEFT_CORNER),
        tile_bottom_right_corner: asset_server.load(assets::tilemap::TILE_BOTTOM_RIGHT_CORNER),
        tile_enter_top_01: asset_server.load(assets::tilemap::TILE_ENTER_TOP_01),
        tile_enter_top_02: asset_server.load(assets::tilemap::TILE_ENTER_TOP_02),
        tile_exit_bottom_01: asset_server.load(assets::tilemap::TILE_EXIT_BOTTOM_01),
        tile_exit_bottom_02: asset_server.load(assets::tilemap::TILE_EXIT_BOTTOM_02),
        tile_outline_top: asset_server.load(assets::tilemap::TILE_OUTLINE_TOP),
        tile_outline_top_left: asset_server.load(assets::tilemap::TILE_OUTLINE_TOP_LEFT),
        tile_outline_top_right: asset_server.load(assets::tilemap::TILE_OUTLINE_TOP_RIGHT),
        tile_outline_bottom: asset_server.load(assets::tilemap::TILE_OUTLINE_BOTTOM),
        tile_outline_bottom_left: asset_server.load(assets::tilemap::TILE_OUTLINE_BOTTOM_LEFT),
        tile_outline_bottom_right: asset_server.load(assets::tilemap::TILE_OUTLINE_BOTTOM_RIGHT),
        tile_outline_left: asset_server.load(assets::tilemap::TILE_OUTLINE_LEFT),
        tile_outline_right: asset_server.load(assets::tilemap::TILE_OUTLINE_RIGHT),
        tile_road_enter: asset_server.load(assets::tilemap::TILE_ROAD_ENTER),
    };
    commands.insert_resource(game_textures);
}

fn main() {
    let event_loop = event_loop::EventLoop::new();

    let monitor = event_loop.available_monitors().nth(0).unwrap().size();
    let monitor_width = monitor.width as f32;
    let monitor_height = monitor.height as f32;

    let player_texture_atlas_config = PlayerTextureAtlasConfig::new(PlayerSprite::OrcWarrior.get_path());

    App::new()
    .insert_resource(player_texture_atlas_config)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Blue Maze".to_string(),
                width: monitor_width,
                height: monitor_height,
                decorations: true,
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
