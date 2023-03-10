use bevy::prelude::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}