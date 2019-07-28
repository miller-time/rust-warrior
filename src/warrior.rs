use crate::level::Level;

/// The protagonist of rust-warrior!
pub struct Warrior {
    pub level: Level,
}

impl Warrior {
    pub fn new(level: Level) -> Warrior {
        Warrior { level }
    }

    pub fn walk(&mut self) {
        self.level.move_warrior();
    }
}
