mod difficulty;

fn main() {

    enum Boxes {
        UP,
        DOWN,
        LEFT,
        RIGHT,
    }

    let chosen_difficulty = difficulty::get_difficulty();
        
    struct Board {
        boxes: Vec<Boxes>,
        player_coordinates: [u16]
    }


}
