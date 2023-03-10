pub enum PlayerSprite {
    Golem01,
    Golem02,
    Golem03,
}


// PATH
impl PlayerSprite {
    pub fn get_path(&self) -> &str {
        match *self {
            PlayerSprite::Golem01 => "golem_01.png",
            PlayerSprite::Golem02 => "golem_02.png",
            PlayerSprite::Golem03 => "golem_03.png",
        }
    }
}

pub const TILE_TOP: &str = "tile_top.png";
pub const TILE_BOTTOM: &str = "tile_bottom.png";
pub const TILE_WALL: &str = "tile_wall.png";
pub const TILE_ROAD: &str = "tile_road.png";

// SIZE
pub const PLAYER_SIZE: (f32, f32) = (410., 560.);
pub const SPRITE_SCALE: f32 = 0.08;

pub const TILE_SIZE: (f32, f32) = (48., 48.);
pub const WALL_SIZE: (f32, f32) = (5., 48.);