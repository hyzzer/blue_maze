use bevy::prelude::*;
use crate::resources::board::{WallStatus, Board};
use crate::resources::game::{BoardSize};
use crate::{GameTextures, WinSize};
use crate::assets::tilemap::{TILE_SIZE, WALL_SIZE};

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, background_spawn_system)
            .add_startup_system_to_stage(StartupStage::PostStartup, tilemap_spawn_system)
            .add_startup_system_to_stage(StartupStage::PostStartup, wallmap_spawn_system)            
            .add_startup_system_to_stage(StartupStage::PostStartup, objects_spawn_system);
    }
}

fn background_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let scale_x = win_size.w / TILE_SIZE.0;
    let scale_y = win_size.h / TILE_SIZE.1;
    tile_spawn_system(&mut commands, game_textures.tile_background.clone(), Vec3::splat(0.), Some(Vec3::new(scale_x, scale_y, 1.)));
}

fn tilemap_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    board_size: Res<BoardSize>,
) {
    let start_x = - (board_size.x as f32 / 2.) * TILE_SIZE.0 - TILE_SIZE.0 / 2.;
    let start_y = (board_size.y as f32 / 2. + 1.) * TILE_SIZE.1 - TILE_SIZE.1 / 2.;
    let mut tile_sprite: Handle<Image>;

    for row_idx in 0..board_size.y + 2 {
        for column_idx in 0..board_size.x + 2 {
            if row_idx == 0 {
                if column_idx == 0 {
                    tile_sprite = game_textures.tile_background.clone();
                }
                else if column_idx == 1 {
                    tile_sprite = game_textures.tile_top_left_corner.clone();
                }
                else {
                    tile_sprite = game_textures.tile_top.clone();
                }
            }
            else if row_idx == 1 {
                if column_idx == 0 {
                    tile_sprite = game_textures.tile_left.clone();
                }
                else if column_idx == 1 {
                    tile_sprite = game_textures.tile_outline_top_left.clone();
                }
                else if column_idx == board_size.x {
                    tile_sprite = game_textures.tile_outline_top_right.clone();
                }
                else if column_idx == board_size.x + 1 {
                    tile_sprite = game_textures.tile_road_enter.clone();
                }
                else {
                    tile_sprite = game_textures.tile_outline_top.clone();
                }
            }
            else if row_idx == board_size.y {
                if column_idx == 0  {
                    tile_sprite = game_textures.tile_road_enter.clone();
                }
                else if column_idx == 1 {
                    tile_sprite = game_textures.tile_outline_bottom_left.clone();
                }
                else if column_idx == board_size.x {
                    tile_sprite = game_textures.tile_outline_bottom_right.clone();
                }
                else if column_idx == board_size.x + 1 {
                    tile_sprite = game_textures.tile_right.clone();
                }
                else {
                    tile_sprite = game_textures.tile_outline_bottom.clone();
                }
            }
            else if row_idx == board_size.y + 1 {
                if column_idx == board_size.x + 1 {
                    tile_sprite = game_textures.tile_background.clone();
                }
                else if column_idx == board_size.x {
                    tile_sprite = game_textures.tile_bottom_right_corner.clone();
                }
                else {
                    tile_sprite = game_textures.tile_bottom.clone();
                }
            }
            else {
                if column_idx == 0 {
                    if row_idx == board_size.y - 1 {
                        tile_sprite = game_textures.tile_enter_top_01.clone();
                    } else {
                        tile_sprite = game_textures.tile_left.clone();
                    }
                }
                else if column_idx == 1 {
                    tile_sprite = game_textures.tile_outline_left.clone();
                }
                else if column_idx == board_size.x {
                    tile_sprite = game_textures.tile_outline_right.clone();
                }
                else if column_idx == board_size.x + 1 {
                    if row_idx == 2 {
                        tile_sprite = game_textures.tile_exit_bottom_01.clone();
                    } else {
                        tile_sprite = game_textures.tile_right.clone();
                    }
                }
                else {
                    tile_sprite = game_textures.tile_road.clone();
                }
            }
            tile_spawn_system(&mut commands, tile_sprite, Vec3::new(start_x + TILE_SIZE.0 * column_idx as f32, start_y - TILE_SIZE.1 * row_idx as f32, 1.), None);
        }
        // CAS PARTICULIERS
        tile_spawn_system(&mut commands, game_textures.tile_enter_top_02.clone(), Vec3::new(start_x - TILE_SIZE.0, start_y - TILE_SIZE.1 * (board_size.y as f32 - 1.), 2.), None);
        tile_spawn_system(&mut commands, game_textures.tile_road_enter.clone(), Vec3::new(start_x - TILE_SIZE.0, start_y - TILE_SIZE.1 * board_size.y as f32, 2.), None);
        tile_spawn_system(&mut commands, game_textures.tile_bottom_left_corner.clone(), Vec3::new(start_x - TILE_SIZE.0, start_y - TILE_SIZE.1 * (board_size.y as f32 + 1.), 2.), None);
        
        tile_spawn_system(&mut commands, game_textures.tile_top_right_corner.clone(), Vec3::new(start_x + TILE_SIZE.0 * (board_size.x as f32 + 2.), start_y, 2.), None);
        tile_spawn_system(&mut commands, game_textures.tile_road_enter.clone(), Vec3::new(start_x + TILE_SIZE.0 * (board_size.x as f32 + 2.), start_y - TILE_SIZE.1, 2.), None);
        tile_spawn_system(&mut commands, game_textures.tile_exit_bottom_02.clone(), Vec3::new(start_x + TILE_SIZE.0 * (board_size.x as f32 + 2.), start_y - TILE_SIZE.1 * 2., 2.), None);
    }
}

fn wallmap_spawn_system(
    mut commands: Commands,
    board: Res<Board>,
    board_size: Res<BoardSize>,
) {
    let start_x = - (board_size.x as f32 / 2.) * TILE_SIZE.0;
    let start_y = (board_size.y as f32 / 2.) * TILE_SIZE.1;

    for row_idx in 0..board_size.y {
        for column_idx in 0..board_size.x {
            if column_idx < board_size.x - 1 {
                if board.vertical_walls[column_idx + (board_size.x - 1) * row_idx] == WallStatus::CLOSED {
                    wall_spawn_system(&mut commands, Vec3::new(start_x + TILE_SIZE.0 * (column_idx as f32 + 1.), start_y - TILE_SIZE.1 / 2. - TILE_SIZE.1 * row_idx as f32, 2.), true);
                }
            }
            if row_idx < board_size.y - 1 {
                if board.horizontal_walls[column_idx + board_size.x * row_idx] == WallStatus::CLOSED {
                    wall_spawn_system(&mut commands, Vec3::new(start_x + TILE_SIZE.0 / 2. + TILE_SIZE.0 * column_idx as f32, start_y - TILE_SIZE.0 * (row_idx as f32 + 1.), 2.), false);
                }
            }  
        }
    }
}

fn objects_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    board_size: Res<BoardSize>,
) {
    tile_spawn_system(&mut commands, game_textures.door_closed.clone(), 
    Vec3::new(
        - (board_size.x as f32 / 2. + 1.5) * TILE_SIZE.0,
        - (board_size.y as f32 / 2. - 0.5) * TILE_SIZE.1,
        2.
    ), 
        Some(Vec3::new(0.09, 0.09, 1.)
    ));
    tile_spawn_system(&mut commands, game_textures.chest_locked.clone(),
    Vec3::new(
        (board_size.x as f32 / 2. + 1.5) * TILE_SIZE.0,
        (board_size.y as f32 / 2. - 0.65) * TILE_SIZE.1,
        2.
    ), 
        Some(Vec3::splat(0.3))
    );
}

fn tile_spawn_system(
    commands: &mut Commands,
    sprite: Handle<Image>, 
    translation: Vec3,
    scale: Option<Vec3>,
) {
    commands.spawn(SpriteBundle {
        texture: sprite,
        transform: Transform {
            translation: translation,
            scale: scale.unwrap_or(Vec3::splat(1.)),
            ..default()
        },
        ..default()
    });
}

fn wall_spawn_system(
    commands: &mut Commands,
    translation: Vec3,
    vertical: bool,
) {
    let wall_dimensions = if vertical {
        Some(Vec2::new(WALL_SIZE.0, WALL_SIZE.1))
    } else {
        Some(Vec2::new(WALL_SIZE.1, WALL_SIZE.0))
    };
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 0., 0.),
            custom_size: wall_dimensions,
            ..default()
        },
        transform: Transform {
            translation: translation,
            ..default()
        },
        ..default()
    });
}