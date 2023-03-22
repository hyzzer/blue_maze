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

// GAME TEXTURES
pub const TILE_TOP: &str = "tile_top.png";
pub const TILE_BOTTOM: &str = "tile_bottom.png";
pub const TILE_ROAD: &str = "tile_road.png";
pub const TILE_LEFT: &str = "tile_left.png";
pub const TILE_RIGHT: &str = "tile_right.png";

pub const TILE_BACKGROUND: &str = "tile_background.png";

pub const TILE_TOP_LEFT_CORNER: &str = "tile_top_left_corner.png";
pub const TILE_TOP_RIGHT_CORNER: &str = "tile_top_right_corner.png";


pub const TILE_BOTTOM_LEFT_CORNER: &str = "tile_bottom_left_corner.png";

pub const TILE_BOTTOM_RIGHT_CORNER: &str = "tile_bottom_right_corner.png";

pub const TILE_ENTER_TOP_01: &str = "tile_enter_top_01.png";
pub const TILE_ENTER_TOP_02: &str = "tile_enter_top_02.png";


pub const TILE_EXIT_BOTTOM_01: &str = "tile_exit_bottom_01.png";
pub const TILE_EXIT_BOTTOM_02: &str = "tile_exit_bottom_02.png";

pub const DOOR_CLOSED: &str = "door_closed.png";
pub const CHEST_CLOSED: &str = "chest_closed.png";


// OUTLINE TILES

pub const TILE_OUTLINE_TOP: &str = "tile_outline_top.png";
pub const TILE_OUTLINE_TOP_LEFT: &str = "tile_outline_top_left.png";
pub const TILE_OUTLINE_TOP_RIGHT: &str = "tile_outline_top_right.png";
pub const TILE_OUTLINE_BOTTOM: &str = "tile_outline_bottom.png";
pub const TILE_OUTLINE_BOTTOM_LEFT: &str = "tile_outline_bottom_left.png";
pub const TILE_OUTLINE_BOTTOM_RIGHT: &str = "tile_outline_bottom_right.png";
pub const TILE_OUTLINE_LEFT: &str = "tile_outline_left.png";
pub const TILE_OUTLINE_RIGHT: &str = "tile_outline_right.png";

pub const TILE_ROAD_ENTER: &str = "tile_road_enter.png";

// SIZE
pub const PLAYER_SIZE: (f32, f32) = (410., 560.);
pub const SPRITE_SCALE: f32 = 0.07;

pub const TILE_SIZE: (f32, f32) = (48., 48.);
pub const WALL_SIZE: (f32, f32) = (5., 50.);