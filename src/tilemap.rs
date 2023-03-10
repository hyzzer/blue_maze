use bevy::prelude::*;
use crate::board::WallStatus;
use crate::{GameTextures, BoardSize, Walls};
use crate::assets::{TILE_SIZE, WALL_SIZE};

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, tilemap_spawn_system);
    }
}

fn tilemap_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    board_size: Res<BoardSize>,
    walls: Res<Walls>,
) {
    let start_left = - (board_size.x as f32 / 2.) * TILE_SIZE.0;
    let start_top = (board_size.y as f32 / 2.) * TILE_SIZE.1;

    // TOP AND BOTTOM
    for tile_idx in 0..board_size.x + 2 {
        tile_spawn_system(&mut commands, game_textures.tile_top.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * tile_idx as f32 - TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. + TILE_SIZE.1, 1.));
        tile_spawn_system(&mut commands, game_textures.tile_bottom.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * tile_idx as f32 - TILE_SIZE.0, - start_top + TILE_SIZE.1 / 2. - TILE_SIZE.1, 1.));
    }
    // LEFT AND RIGHT
    for tile_idx in 0..board_size.y {
        if tile_idx != board_size.y - 1 {
            tile_spawn_system(&mut commands, game_textures.tile_wall.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * tile_idx as f32, 1.));
        }
        else {
            tile_spawn_system(&mut commands, game_textures.tile_road.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * tile_idx as f32, 1.));
        }
        if tile_idx != 0 {
            tile_spawn_system(&mut commands, game_textures.tile_wall.clone(), Vec3::new(- start_left - TILE_SIZE.0 / 2. + TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * tile_idx as f32, 1.));        
        }
        else {
            tile_spawn_system(&mut commands, game_textures.tile_road.clone(), Vec3::new(- start_left - TILE_SIZE.0 / 2. + TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * tile_idx as f32, 1.));        
        }
    }
    for row_idx in 1..board_size.x - 1 {
        for column_idx in 1..board_size.y - 1 {
            tile_spawn_system(&mut commands, game_textures.tile_road.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * column_idx as f32, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * row_idx as f32, 1.));
            // VERTICAL WALLS
            if column_idx != 1 {
                if walls.vertical_walls[column_idx - 2 + (board_size.x - 1) * (row_idx - 1)] == WallStatus::OPEN{
                    wall_spawn_system(&mut commands, Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * column_idx as f32 - WALL_SIZE.0 / 2., start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * row_idx as f32, 3.), true);
                }
            }
        }
    }
}

fn tile_spawn_system(
    commands: &mut Commands,
    sprite: Handle<Image>, 
    translation: Vec3,
) {
    commands.spawn(SpriteBundle {
        texture: sprite,
        transform: Transform {
            translation: translation,
            // scale
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
        Some(Vec2::new(5., 48.))
    } else {
        Some(Vec2::new(48., 5.))
    };
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
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