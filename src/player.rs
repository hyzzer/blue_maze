use bevy::prelude::*;
use bevy::time::FixedTimestep;
use crate::assets::{PLAYER_SIZE, SPRITE_SCALE};
use crate::components::{Player, Position};
use crate::game::{SPEED};
use crate::{GameTextures, WinSize};

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
    win_size: Res<WinSize>
) {
    let bottom = - win_size.h / 2.;

    commands.spawn(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE, 2.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    })
    .insert(Player)
    .insert(Position {
        x: 0,
        y: 0,
    });
}

fn player_keyboard_event_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        if keyboard.pressed(KeyCode::Up) {
            transform.translation.y += SPEED;
        }
        if keyboard.pressed(KeyCode::Down) {
            transform.translation.y -= SPEED;
        }
        if keyboard.pressed(KeyCode::Right) {
            transform.translation.x += SPEED;
        }
        if keyboard.pressed(KeyCode::Left) {
            transform.translation.x -= SPEED;
        }
    }
}