use bevy::prelude::*;
use crate::resources::animations::PlayerTextureAtlasConfig;

#[derive(Component)]
pub struct PlayerAnimations {
    pub size: Vec2,
    pub idle_left: Handle<TextureAtlas>,
    pub idle_right: Handle<TextureAtlas>,
    pub run_left: Handle<TextureAtlas>,
    pub run_right: Handle<TextureAtlas>,
    pub open_chest: Handle<TextureAtlas>,
}

pub fn build_player_animations(
    player_texture_atlas_config: & Res<PlayerTextureAtlasConfig>,
    texture_atlases: &mut ResMut<'_, Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
) -> PlayerAnimations {    
    let player_texture_atlas_idle_left = TextureAtlas::from_grid(
        asset_server.load(&player_texture_atlas_config.animations.idle_left.texture_path),
        player_texture_atlas_config.tile_size,
        player_texture_atlas_config.animations.idle_left.len,
        1,
        Some(Vec2::splat(0.)),
        Some(Vec2::splat(0.)),
    );
    let player_texture_atlas_handle_idle_left = texture_atlases.add(player_texture_atlas_idle_left);
    
    let player_texture_atlas_idle_right = TextureAtlas::from_grid(
        asset_server.load(&player_texture_atlas_config.animations.idle_right.texture_path),
        player_texture_atlas_config.tile_size,
        player_texture_atlas_config.animations.idle_right.len,
        1,
        Some(Vec2::splat(0.)),
        Some(Vec2::splat(0.)),
    );
    let player_texture_atlas_handle_idle_right = texture_atlases.add(player_texture_atlas_idle_right);

    let player_texture_atlas_run_left = TextureAtlas::from_grid(
        asset_server.load(&player_texture_atlas_config.animations.run_left.texture_path),
        player_texture_atlas_config.tile_size,
        player_texture_atlas_config.animations.run_left.len,
        1,
        Some(Vec2::splat(0.)),
        Some(Vec2::splat(0.)),
    );
    let player_texture_atlas_handle_run_left = texture_atlases.add(player_texture_atlas_run_left);

    let player_texture_atlas_run_right = TextureAtlas::from_grid(
        asset_server.load(&player_texture_atlas_config.animations.run_right.texture_path),
        player_texture_atlas_config.tile_size,
        player_texture_atlas_config.animations.run_right.len,
        1,
        Some(Vec2::splat(0.)),
        Some(Vec2::splat(0.)),
    );
    let player_texture_atlas_handle_run_right = texture_atlases.add(player_texture_atlas_run_right);

    let player_texture_atlas_open_chest = TextureAtlas::from_grid(
        asset_server.load(&player_texture_atlas_config.animations.open_chest.texture_path),
        player_texture_atlas_config.tile_size,
        player_texture_atlas_config.animations.open_chest.len,
        1,
        Some(Vec2::splat(0.)),
        Some(Vec2::splat(0.)),

    );
    let player_texture_atlas_handle_open_chest = texture_atlases.add(player_texture_atlas_open_chest);

    PlayerAnimations {
        size: player_texture_atlas_config.tile_size,
        idle_left: player_texture_atlas_handle_idle_left,
        idle_right: player_texture_atlas_handle_idle_right,
        run_left: player_texture_atlas_handle_run_left,
        run_right: player_texture_atlas_handle_run_right,
        open_chest: player_texture_atlas_handle_open_chest,
    }
}