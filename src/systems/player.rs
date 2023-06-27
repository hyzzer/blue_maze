use bevy::prelude::*;
use crate::resources::animations::{PlayerTextureAtlasConfig};
use crate::assets::tilemap::{TILE_SIZE};
use crate::components::player::{Player, PlayerState, CurrentAnimation, CurrentMove};
use crate::resources::game::BoardSize;
use crate::components::player_animations::{build_player_animations, PlayerAnimations};
use crate::resources::board::{Direction, Board, WallStatus};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(animate_player_system)
            .add_system(move_player_system)
            .add_system(player_keyboard_event_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    board_size: Res<BoardSize>,
    player_texture_atlas_config: Res<PlayerTextureAtlasConfig>,
) {
    let player_animations = build_player_animations(&player_texture_atlas_config, &mut texture_atlases, &asset_server);

    let start_player = Vec3::new(
        - (board_size.x as f32 / 2.) * TILE_SIZE.0 - TILE_SIZE.0 / 2.,
        - (board_size.y as f32 / 2.) * TILE_SIZE.1 + TILE_SIZE.1 / 2.,
        4.,
    );
    let player_tile_size = &player_animations.size[0];

    let sprite_scale = (TILE_SIZE.1 / 0.7) / player_tile_size;

    commands.spawn(SpriteSheetBundle{
        texture_atlas: player_animations.idle_right.clone(),
        transform: Transform {
            translation: start_player,
            scale: Vec3::new(sprite_scale, sprite_scale, 1.),
            ..default()
        },
        ..default()
    }) 
    .insert(Player {
        position: -1 as isize,
        state: PlayerState::IdleRight,
    })
    .insert(CurrentAnimation {
        timer: Timer::from_seconds(0.025, TimerMode::Repeating),
        frame: 0,
    })
    .insert(player_animations);
}


fn animate_player_system(
    time: Res<Time>,
    player_texture_atlas_config: Res<PlayerTextureAtlasConfig>,
    mut query: Query<(&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &mut CurrentAnimation, &mut PlayerAnimations, &mut Player), With<Player>>,
) {
    if let Ok((mut texture_atlas_handle, mut player_sprite, mut current_animation, player_animations, player)) = query.get_single_mut() {
        current_animation.timer.tick(time.delta());
        if current_animation.timer.finished() {
            let animation_len = match player.state {
                PlayerState::IdleLeft => {
                    *texture_atlas_handle = player_animations.idle_left.clone();
                    player_texture_atlas_config.animations.idle_left.len
                },
                PlayerState::IdleRight => {
                    *texture_atlas_handle = player_animations.idle_right.clone();
                    player_texture_atlas_config.animations.idle_right.len
                }
                PlayerState::RunLeft => {
                    *texture_atlas_handle = player_animations.run_left.clone();
                    player_texture_atlas_config.animations.run_left.len
                },
                PlayerState::RunRight => {
                    *texture_atlas_handle = player_animations.run_right.clone();
                    player_texture_atlas_config.animations.run_right.len
                },
                PlayerState::OpenChest => {
                    *texture_atlas_handle = player_animations.open_chest.clone();
                    player_texture_atlas_config.animations.open_chest.len
                },
            };
            current_animation.frame = (current_animation.frame + 1) % animation_len;
            player_sprite.index = current_animation.frame;

            current_animation.timer.reset();
        }
    }
}

fn move_player_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player, &mut CurrentMove), With<Player>>,
) {
    if let Ok((mut transform, mut player, mut current_move)) = query.get_single_mut() {
        current_move.timer.tick(time.delta());

        let progress = current_move.timer.elapsed().as_secs_f32() / current_move.timer.duration().as_secs_f32();
        transform.translation = current_move.start * (1.0 - progress) + current_move.end * progress;

        if current_move.timer.just_finished() {
            player.state = match player.state {
                PlayerState::IdleLeft => PlayerState::IdleLeft,
                PlayerState::IdleRight => PlayerState::IdleRight,
                PlayerState::RunLeft => PlayerState::IdleLeft,
                PlayerState::RunRight => PlayerState::IdleRight,
                PlayerState::OpenChest => PlayerState::IdleRight,
            };
        }
    }
}

fn player_keyboard_event_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    board: Res<Board>,
    board_size: Res<BoardSize>,
    mut query: Query<(Entity, &mut Transform, &mut Player, &mut CurrentAnimation), With<Player>>,
) {
    if let Ok((player_entity, transform, mut player, mut current_animation)) = query.get_single_mut() {       
        let direction: Direction;
        let translation: (isize, isize);
        if player.state == PlayerState::IdleLeft || player.state == PlayerState::IdleRight {
            if keyboard.any_just_pressed([KeyCode::Up, KeyCode::W, KeyCode::Z]) {
                direction = Direction::UP;
                translation = (0, - 1);
                player.state = match player.state {
                    PlayerState::IdleLeft => PlayerState::RunLeft,
                    PlayerState::IdleRight => PlayerState::RunRight,
                    PlayerState::RunLeft => PlayerState::RunLeft,
                    PlayerState::RunRight => PlayerState::RunRight,
                    PlayerState::OpenChest => PlayerState::IdleRight,
                };
                current_animation.timer.reset();
            }
            else if keyboard.any_just_pressed([KeyCode::Down, KeyCode::S]) {
                direction = Direction::DOWN;
                translation = (0, 1);
                player.state = match player.state {
                    PlayerState::IdleLeft => PlayerState::RunLeft,
                    PlayerState::IdleRight => PlayerState::RunRight,
                    PlayerState::RunLeft => PlayerState::RunLeft,
                    PlayerState::RunRight => PlayerState::RunRight,
                    PlayerState::OpenChest => PlayerState::IdleRight,
                };
                current_animation.timer.reset();
            }
            else if keyboard.any_just_pressed([KeyCode::Left, KeyCode::A, KeyCode::Q]) {
                direction = Direction::LEFT;
                translation = (- 1, 0);
                player.state = PlayerState::RunLeft;
                current_animation.timer.reset();
            }
            else if keyboard.any_just_pressed([KeyCode::Right, KeyCode::D]) {
                direction = Direction::RIGHT;
                translation = (1, 0);
                player.state = PlayerState::RunRight;
                current_animation.timer.reset();
            }
            else {
                return;
            }
            // START
            if player.position == -1 && direction == Direction::RIGHT {
                player.position = (board_size.x * (board_size.y - 1)) as isize;
                commands.entity(player_entity).insert(CurrentMove {
                    start: transform.translation,
                    end: transform.translation + Vec3::new(TILE_SIZE.0, 0., 0.),
                    timer: Timer::from_seconds(0.2, TimerMode::Once)
                });


                return;
            }
            // END
            if player.position == (board_size.x - 1) as isize && direction == Direction::RIGHT {
                commands.entity(player_entity).insert(CurrentMove {
                    start: transform.translation,
                    end: transform.translation + Vec3::new(TILE_SIZE.0, 0., 0.),
                    timer: Timer::from_seconds(0.1, TimerMode::Once)
                });

                player.state = PlayerState::OpenChest;

                println!("GAGNER");
                return;
            }

            if board.is_wall_open(player.position as usize, &direction) == WallStatus::OPEN {
                println!("OPEN");

                commands.entity(player_entity).insert(CurrentMove {
                    start: transform.translation,
                    end: transform.translation + Vec3::new(translation.0 as f32 * TILE_SIZE.0, -translation.1 as f32 * TILE_SIZE.1, 0.),
                    timer: Timer::from_seconds(0.2, TimerMode::Once)
                });

                player.position += translation.0 + translation.1 * board_size.x as isize;
            }
            else {
                player.state = match player.state {
                    PlayerState::IdleLeft => PlayerState::IdleLeft,
                    PlayerState::IdleRight => PlayerState::IdleRight,
                    PlayerState::RunLeft => PlayerState::IdleLeft,
                    PlayerState::RunRight => PlayerState::IdleRight,
                    PlayerState::OpenChest => PlayerState::IdleRight,
                };
                println!("CLOSED");
            }
            println!("translation : ({}, {})", translation.0, translation.1);
            println!("position : {}\n", player.position);
        }
    }
}