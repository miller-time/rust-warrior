use crate::{floor::Floor, unit::Unit, Player};

/// The mutating game state managed by the engine.
pub struct World {
    pub player_name: String,
    pub warrior_level: usize,
    pub floor: Floor,
    pub player: Box<dyn Player + Send + Sync>,
    pub warrior: Unit,
    pub other_units: Vec<Unit>,
}

impl World {
    pub fn new(
        player_name: String,
        warrior_level: usize,
        floor: Floor,
        player: Box<dyn Player + Send + Sync>,
        warrior: Unit,
        other_units: Vec<Unit>,
    ) -> World {
        World {
            player_name,
            warrior_level,
            floor,
            player,
            warrior,
            other_units,
        }
    }

    pub fn remove_unit(&mut self, index: usize) {
        self.other_units.remove(index);
    }
}
