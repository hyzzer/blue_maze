use bevy::prelude::*;
use bevy::time::FixedTimestep;
use crate::assets::{SPRITE_SCALE, TILE_SIZE};
use crate::components::{Player, Position};
use crate::game::BoardSize;
use crate::{GameTextures};
use crate::board::{Direction, Board, WallStatus};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.10))
                    .with_system(player_keyboard_event_system),
            );
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    board_size: Res<BoardSize>,
) {
    let start_position = Vec3::new(
        - (board_size.x as f32 / 2.) * TILE_SIZE.0 - TILE_SIZE.0 / 2.,
        - (board_size.y as f32 / 2.) * TILE_SIZE.1 + TILE_SIZE.1 / 2.,
        4.,
    );
    commands.spawn(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: start_position,
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    })
    .insert(Player)
    .insert(Position {
        value: board_size.x * board_size.y,
    });
}

fn player_keyboard_event_system(
    keyboard: Res<Input<KeyCode>>,
    board: Res<Board>,
    board_size: Res<BoardSize>,
    mut query: Query<(&mut Transform, &mut Position), With<Position>>,
) {
    if let Ok((mut transform, mut position)) = query.get_single_mut() {       
        let direction: Direction;
        let translation: (f32, f32);
        if keyboard.pressed(KeyCode::Up) {
            direction = Direction::UP;
            translation = (0., 1.);
        }
        else if keyboard.pressed(KeyCode::Down) {
            direction = Direction::DOWN;
            translation = (0., - 1.);
        }
        else if keyboard.pressed(KeyCode::Right) {
            direction = Direction::RIGHT;
            translation = (1., 0.);
        }
        else if keyboard.pressed(KeyCode::Left) {
            direction = Direction::LEFT;
            translation = (- 1., 0.);
        }
        else {
            return;
        }
        println!("{} : {}", translation.0, translation.1);
        if board.is_wall_open(position.value, &direction) == WallStatus::OPEN {
            transform.translation.x += translation.0 * TILE_SIZE.0;
            transform.translation.y += translation.1 * TILE_SIZE.1;
            if position.value == board_size.x * board_size.y {
                position.value = board_size.x * (board_size.y - 1);
            } else {
                position.value += translation.0 as usize + (translation.1 as usize * board_size.x);
            }
        }
    }
}