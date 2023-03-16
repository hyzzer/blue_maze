use bevy::prelude::*;
use crate::board::{WallStatus, Board};
use crate::game::{BoardSize};
use crate::{GameTextures};
use crate::assets::{TILE_SIZE};

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
    board: Res<Board>,
    board_size: Res<BoardSize>,
) {
    let start_left = - (board_size.x as f32 / 2.) * TILE_SIZE.0;
    let start_top = (board_size.y as f32 / 2.) * TILE_SIZE.1;

    // LINES ON TOP AND BOTTOM
    tile_spawn_system(&mut commands, game_textures.tile_background.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. + TILE_SIZE.1, 3.), None);
    tile_spawn_system(&mut commands, game_textures.tile_background.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * (board_size.x as f32 + 1.) - TILE_SIZE.0, - start_top + TILE_SIZE.1 / 2. - TILE_SIZE.1, 3.), None);

    tile_spawn_system(&mut commands, game_textures.tile_top.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * (board_size.x as f32 + 1.) - TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. + TILE_SIZE.1, 3.), None);
    tile_spawn_system(&mut commands, game_textures.tile_top_right_corner_02.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * (board_size.x as f32 + 2.) - TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. + TILE_SIZE.1, 3.), None);

    tile_spawn_system(&mut commands, game_textures.tile_bottom_left_corner_02.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0 * 2., - start_top + TILE_SIZE.1 / 2. - TILE_SIZE.1, 3.), None);
    tile_spawn_system(&mut commands, game_textures.tile_bottom.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0, - start_top + TILE_SIZE.1 / 2. - TILE_SIZE.1, 3.), None);
    
    for tile_idx in 0..2 {
        tile_spawn_system(&mut commands, game_textures.tile_background.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0 * (tile_idx as f32 + 1.), start_top - TILE_SIZE.1 / 2. + TILE_SIZE.1, 3.), None);
    }

    // BACKGROUNDS ABOVE AND BELOW MAP
    for row_idx in 0..4 {
        for column_idx in 0..board_size.x + 4 {
            tile_spawn_system(&mut commands, game_textures.tile_background.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * column_idx as f32 - TILE_SIZE.0 * 2., start_top - TILE_SIZE.1 / 2. + TILE_SIZE.1 * (row_idx as f32 + 2.), 3.), None);
            tile_spawn_system(&mut commands, game_textures.tile_background.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * column_idx as f32 - TILE_SIZE.0 * 2., - start_top + TILE_SIZE.1 / 2. - TILE_SIZE.1 * (row_idx as f32 + 2.), 3.), None);
        }
    }


    // OUTSIDE WALLS
    for tile_idx in 1..board_size.x + 1 {
        if tile_idx == 1 {
            tile_spawn_system(&mut commands, game_textures.tile_top_left_corner.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * tile_idx as f32 - TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. + TILE_SIZE.1, 3.), None);
        }
        else {
            tile_spawn_system(&mut commands, game_textures.tile_top.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * tile_idx as f32 - TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. + TILE_SIZE.1, 3.), None);
        }
        if tile_idx == board_size.x {
            tile_spawn_system(&mut commands, game_textures.tile_bottom_right_corner.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * tile_idx as f32 - TILE_SIZE.0, - start_top + TILE_SIZE.1 / 2. - TILE_SIZE.1, 3.), None);
        }
        else {
            tile_spawn_system(&mut commands, game_textures.tile_bottom.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * tile_idx as f32 - TILE_SIZE.0, - start_top + TILE_SIZE.1 / 2. - TILE_SIZE.1, 3.), None);
        }
    }
    // LEFT AND RIGHT
    for tile_idx in 0..board_size.y {
        tile_spawn_system(&mut commands, game_textures.tile_background.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0 * 2., start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * (tile_idx as f32 - 1.), 1.), None);
        tile_spawn_system(&mut commands, game_textures.tile_background.clone(), Vec3::new(- start_left - TILE_SIZE.0 / 2. + TILE_SIZE.0 * 2., start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * (tile_idx as f32 + 1.), 1.), None);        
        
        
        if tile_idx == board_size.y - 1 {
            tile_spawn_system(&mut commands, game_textures.tile_road_enter.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * tile_idx as f32, 1.), None);
        }
        else if tile_idx == board_size.y - 2 {
            tile_spawn_system(&mut commands, game_textures.tile_enter_top_01.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * tile_idx as f32, 1.), None);
        } else {
            tile_spawn_system(&mut commands, game_textures.tile_left_01.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * tile_idx as f32, 3.), None);
        }
        if tile_idx == 0 {
            tile_spawn_system(&mut commands, game_textures.tile_road_enter.clone(), Vec3::new(- start_left - TILE_SIZE.0 / 2. + TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * tile_idx as f32, 1.), None);        
        }
        else if tile_idx ==  1 {
            tile_spawn_system(&mut commands, game_textures.tile_exit_bottom.clone(), Vec3::new(- start_left - TILE_SIZE.0 / 2. + TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * tile_idx as f32, 1.), None);        
        }
        else {
            tile_spawn_system(&mut commands, game_textures.tile_right_01.clone(), Vec3::new(- start_left - TILE_SIZE.0 / 2. + TILE_SIZE.0, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * tile_idx as f32, 3.), None);        
        }
    
    // ENTER AND EXIT
    tile_spawn_system(&mut commands, game_textures.tile_road_enter.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0 * 2., start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * (board_size.y as f32 - 1.), 1.), None);
    tile_spawn_system(&mut commands, game_textures.door_closed.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0 * 2., start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * (board_size.y as f32 - 1.), 2.), Some(Vec3::new(0.09, 0.09, 1.)));

    tile_spawn_system(&mut commands, game_textures.tile_road_enter.clone(), Vec3::new(- start_left - TILE_SIZE.0 / 2. + TILE_SIZE.0 * 2., start_top - TILE_SIZE.1 / 2., 1.), None);        
    tile_spawn_system(&mut commands, game_textures.chest_closed.clone(), Vec3::new(- start_left - TILE_SIZE.0 / 2. + TILE_SIZE.0 * 2., start_top - TILE_SIZE.1 / 2., 2.), Some(Vec3::splat(0.3)));        

    // ABOVE THE ENTER
    tile_spawn_system(&mut commands, game_textures.tile_enter_top_02.clone(), Vec3::new(start_left + TILE_SIZE.0 / 2. - TILE_SIZE.0 * 2., start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * (board_size.y as f32 - 2.), 1.), None);
    
    // BELOW THE EXIT
    tile_spawn_system(&mut commands, game_textures.tile_exit_below.clone(), Vec3::new(- start_left - TILE_SIZE.0 / 2. + TILE_SIZE.0 * 2., start_top - TILE_SIZE.1 / 2. - TILE_SIZE.1, 2.), None);        



    // INSIDE WALLS
    }
    for row_idx in 0..board_size.y {
        for column_idx in 0..board_size.x {
            let tile_sprite: Handle<Image>;
            if row_idx == 0 {
                if column_idx == 0 {
                    tile_sprite = game_textures.tile_outline_top_left.clone();
                } else if column_idx == board_size.x - 1 {
                    tile_sprite = game_textures.tile_outline_top_right.clone();
                } else {
                    tile_sprite = game_textures.tile_outline_top.clone();
                }
            } else if row_idx == board_size.y - 1 {
                if column_idx == 0 {
                    tile_sprite = game_textures.tile_outline_bottom_left.clone();
                } else if column_idx == board_size.x - 1 {
                    tile_sprite = game_textures.tile_outline_bottom_right.clone();
                } else {
                    tile_sprite = game_textures.tile_outline_bottom.clone();
                }
            } else {
                if column_idx == 0 {
                    tile_sprite = game_textures.tile_outline_left.clone();
                } else if column_idx == board_size.x - 1 {
                    tile_sprite = game_textures.tile_outline_right.clone();
                } else {
                    tile_sprite = game_textures.tile_road.clone();
                }
            }
            tile_spawn_system(&mut commands, tile_sprite, Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * column_idx as f32, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * row_idx as f32, 1.), None);
            // VERTICAL WALLS
            if column_idx < board_size.x - 1 {
                if board.vertical_walls[column_idx + (board_size.x - 1) * row_idx] == WallStatus::CLOSED {
                    wall_spawn_system(&mut commands, Vec3::new(start_left + TILE_SIZE.0 * (column_idx + 1) as f32, start_top - TILE_SIZE.1 / 2. - TILE_SIZE.0 * row_idx as f32, 2.), true);
                }
            }
            // HORIZONTAL WALLS
            if row_idx < board_size.y - 1 {
                if board.horizontal_walls[column_idx + board_size.x * row_idx] == WallStatus::CLOSED {
                    wall_spawn_system(&mut commands, Vec3::new(start_left + TILE_SIZE.0 / 2. + TILE_SIZE.0 * column_idx as f32, start_top - TILE_SIZE.0 * (row_idx as f32 + 1.), 2.), false);
                }
            }
        }
    }
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
        Some(Vec2::new(5., 50.))
    } else {
        Some(Vec2::new(50., 5.))
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