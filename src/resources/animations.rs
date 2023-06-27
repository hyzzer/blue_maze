use bevy::prelude::*;
use serde::{Deserialize};
use std::{fs::File};
use ron;

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct AtlasConfig {
    pub texture_path: String,
    pub len: usize,
}

#[derive(Debug, Deserialize)]
pub struct AnimationConfig {
    pub idle_left: AtlasConfig,
    pub idle_right: AtlasConfig,
    pub run_left: AtlasConfig,
    pub run_right: AtlasConfig,
    pub open_chest: AtlasConfig,
}

#[derive(Debug, Deserialize, Resource)]
pub struct PlayerTextureAtlasConfig {
    pub tile_size: Vec2,
    pub animations: AnimationConfig,
}

impl PlayerTextureAtlasConfig {
    pub fn new(animations_config_path: &str) -> PlayerTextureAtlasConfig {
        let animations_file = File::open(animations_config_path).expect("Failed to read animations config file");
        ron::de::from_reader(animations_file).expect("Failed to deserialize animations config file")
    }
}
