use bevy::prelude::*;
use crate::assets::{SPRITE_SCALE, TILE_SIZE};
use crate::components::Player;
use crate::game::BoardSize;
use crate::{GameTextures};
use crate::board::{Direction, Board, WallStatus};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_keyboard_event_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    board_size: Res<BoardSize>,
) {
    let start_player = Vec3::new(
        - (board_size.x as f32 / 2.) * TILE_SIZE.0 - TILE_SIZE.0 / 2.,
        - (board_size.y as f32 / 2.) * TILE_SIZE.1 + TILE_SIZE.1 / 2.,
        4.,
    );
    commands.spawn(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: start_player,
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    })
    .insert(Player {
        position: -1 as isize,
    });
}

fn player_keyboard_event_system(
    keyboard: Res<Input<KeyCode>>,
    board: Res<Board>,
    board_size: Res<BoardSize>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    if let Ok((mut transform, mut player)) = query.get_single_mut() {       
        let direction: Direction;
        let translation: (isize, isize);
        if keyboard.any_just_pressed([KeyCode::Up, KeyCode::W, KeyCode::Z]) {
            direction = Direction::UP;
            translation = (0, - 1);
        }
        else if keyboard.any_just_pressed([KeyCode::Down, KeyCode::S]) {
            direction = Direction::DOWN;
            translation = (0, 1);
        }
        else if keyboard.any_just_pressed([KeyCode::Left, KeyCode::A, KeyCode::Q]) {
            direction = Direction::LEFT;
            translation = (- 1, 0);
        }
        else if keyboard.any_just_pressed([KeyCode::Right, KeyCode::D]) {
            direction = Direction::RIGHT;
            translation = (1, 0);
        }
        else {
            return;
        }
        if player.position == -1 && direction == Direction::RIGHT {
            player.position = (board_size.x * (board_size.y - 1)) as isize;
            transform.translation.x += TILE_SIZE.0;
            return;
        }
        if player.position == (board_size.x - 1) as isize && direction == Direction::RIGHT {
            transform.translation.x += TILE_SIZE.0;
            println!("GAGNER");
            return;
        }

        if board.is_wall_open(player.position as usize, &direction) == WallStatus::OPEN {
            println!("OPEN");
            transform.translation.x += translation.0 as f32 * TILE_SIZE.0;
            transform.translation.y += - translation.1 as f32 * TILE_SIZE.1;
            player.position += translation.0 + translation.1 * board_size.x as isize;
        }
        else {
            println!("CLOSED");
        }
        println!("translation : ({}, {})", translation.0, translation.1);
        println!("position : {}\n", player.position);

    }
}