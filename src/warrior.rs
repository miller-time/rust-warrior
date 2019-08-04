//! contains the interface exposed to the player for controlling the Warrior

use crate::actions::Action;

/// An interface the player can interact with to control the Warrior in the
/// game. An instance is passed to [`Player`](crate::player::Player) via the
/// `play_turn` method.
pub struct Warrior {
    path_clear: bool,
    captive_found: bool,
    health: i32,
    pub action: Option<Action>,
}

impl Warrior {
    pub fn new(path_clear: bool, captive_found: bool, health: i32) -> Warrior {
        Warrior {
            path_clear,
            captive_found,
            health,
            action: None,
        }
    }

    /// Walk forward one tile.
    pub fn walk(&mut self) {
        self.perform(Action::Walk);
    }

    /// Check if the tile in front of the Warrior is clear.
    pub fn path_clear(&self) -> bool {
        self.path_clear
    }

    /// Check if the tile in fron of the Warrior is a Captive.
    pub fn captive_found(&self) -> bool {
        self.captive_found
    }

    /// Attempt to attack an enemy in the tile in front of the Warrior.
    pub fn attack(&mut self) {
        self.perform(Action::Attack);
    }

    /// Check the current health of the Warrior.
    pub fn health(&self) -> i32 {
        self.health
    }

    /// Rest and regain 10% of the Warrior's HP.
    pub fn rest(&mut self) {
        self.perform(Action::Rest);
    }

    /// Attempt to rescue a Captive in front of the Warrior.
    pub fn rescue(&mut self) {
        self.perform(Action::Rescue);
    }

    fn perform(&mut self, action: Action) {
        if self.action.is_some() {
            println!("Warrior already performed action!");
            return;
        }

        self.action = Some(action);
    }
}
