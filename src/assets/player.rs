pub enum PlayerSprite {
    OrcWarrior,
}

// PATH
impl PlayerSprite {
    pub fn get_path(&self) -> &str {
        match *self {
            PlayerSprite::OrcWarrior => "assets/animations/orc_warrior.ron",
        }
    }
}