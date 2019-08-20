//! contains the interface exposed to the player for controlling the Warrior

use crate::{
    actions::{Action, Direction},
    floor::Tile,
};

/// An interface the player can interact with to control the Warrior in the
/// game. An instance is passed to [`Player`](crate::player::Player) via the
/// `play_turn` method.
pub struct Warrior {
    ahead: Vec<Tile>,
    behind: Vec<Tile>,
    health: i32,
    facing: Direction,
    pub action: Option<Action>,
}

impl Warrior {
    pub fn new(ahead: Vec<Tile>, behind: Vec<Tile>, health: i32, facing: Direction) -> Warrior {
        Warrior {
            ahead,
            behind,
            health,
            facing,
            action: None,
        }
    }

    /// Walk forward one tile.
    pub fn walk(&mut self) {
        self.walk_toward(Direction::Forward);
    }

    /// Walk one tile toward specified `direction`.
    pub fn walk_toward(&mut self, direction: Direction) {
        self.perform(Action::Walk(direction));
    }

    /// Check the tile in front of the Warrior.
    /// Returns a [`Tile`](crate::Tile).
    pub fn check(&self) -> Tile {
        self.check_toward(Direction::Forward)
    }

    /// Check the tile toward specified `direction`.
    /// Returns a [`Tile`](crate::Tile).
    pub fn check_toward(&self, direction: Direction) -> Tile {
        match direction {
            Direction::Forward => match self.ahead.first() {
                Some(tile) => *tile,
                None => Tile::Wall,
            },
            Direction::Backward => match self.behind.first() {
                Some(tile) => *tile,
                None => Tile::Wall,
            },
        }
    }

    /// Check three tiles in front of the Warrior.
    /// Returns a vector of up to three [`Tile`](crate::Tile)s.
    pub fn look(&self) -> &Vec<Tile> {
        self.look_toward(Direction::Forward)
    }

    /// Check three tiles toward specified `direction`.
    /// Returns a vector of up to three [`Tile`](crate::Tile)s.
    pub fn look_toward(&self, direction: Direction) -> &Vec<Tile> {
        match direction {
            Direction::Forward => &self.ahead,
            Direction::Backward => &self.behind,
        }
    }

    /// Attempt to attack an enemy in the tile in front of the Warrior.
    pub fn attack(&mut self) {
        self.attack_toward(Direction::Forward);
    }

    /// Attempt to attack an enemy one tile away in specified `direction`.
    pub fn attack_toward(&mut self, direction: Direction) {
        self.perform(Action::Attack(direction));
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
        self.rescue_toward(Direction::Forward);
    }

    /// Attempt to rescue a Captive one tile away in specified `direction`.
    pub fn rescue_toward(&mut self, direction: Direction) {
        self.perform(Action::Rescue(direction));
    }

    /// Rotate 180 degrees
    pub fn pivot(&mut self) {
        let direction = match self.facing {
            Direction::Forward => Direction::Backward,
            Direction::Backward => Direction::Forward,
        };
        self.perform(Action::Pivot(direction));
    }

    /// Fire an arrow up to three tiles in front of the Warrior.
    pub fn shoot(&mut self) {
        self.shoot_toward(Direction::Forward);
    }

    /// Fire an arrow up to three tiles toward specified `direction`.
    pub fn shoot_toward(&mut self, direction: Direction) {
        self.perform(Action::Shoot(direction));
    }

    fn perform(&mut self, action: Action) {
        if self.action.is_some() {
            println!("Warrior already performed action!");
            return;
        }

        self.action = Some(action);
    }
}
