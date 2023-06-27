use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub position: isize,
    pub state: PlayerState,
}

#[derive(PartialEq)]
pub enum PlayerState {
    IdleLeft,
    IdleRight,
    RunLeft,
    RunRight,
    OpenChest,
}

#[derive(Component)]
pub struct CurrentAnimation {
    pub timer: Timer,
    pub frame: usize,
}

#[derive(Component)]
pub struct CurrentMove {
    pub start: Vec3,
    pub end: Vec3,
    pub timer: Timer,
}