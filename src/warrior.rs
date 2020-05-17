//! contains the interface exposed to the player for controlling the Warrior

use crate::{
    actions::{Action, Direction},
    floor::Tile,
};
use std::cell::RefCell;

/// An interface the player can interact with to control the Warrior in the
/// game. An instance is passed to [`Player`](crate::player::Player) via the
/// `play_turn` method.
/// The player must pick one [`Action`](crate::actions::Action) to perform
/// each turn. Not all abilities are an `Action`.
/// Warrior abilities are unlocked as the player progresses through the levels.
///
/// ### Level Guide
///
/// **Level 1**
///
/// Available abilities:
///
/// * [`walk`](crate::warrior::Warrior::walk)
///
/// **Level 2**
///
/// New abilities unlocked at this level:
///
/// * [`check`](crate::warrior::Warrior::check)
/// * [`attack`](crate::warrior::Warrior::attack)
///
/// **Level 3**
///
/// New abilities unlocked at this level:
///
/// * [`health`](crate::warrior::Warrior::health)
/// * [`rest`](crate::warrior::Warrior::rest)
///
/// **Level 4**
///
/// *No new abilities unlocked at this level!*
///
/// **Level 5**
///
/// New abilities unlocked at this level:
///
/// * [`rescue`](crate::warrior::Warrior::rescue)
///
/// **Level 6**
///
/// The following abilities now have a *directional* counterpart:
///
/// * `walk` -> [`walk_toward`](crate::warrior::Warrior::walk_toward)
/// * `check` -> [`check_toward`](crate::warrior::Warrior::check_toward)
/// * `attack` -> [`attack_toward`](crate::warrior::Warrior::attack_toward)
/// * `rescue` -> [`rescue_toward`](crate::warrior::Warrior::rescue_toward)
///
/// **Level 7**
///
/// New abilities unlocked at this level:
///
/// * [`pivot`](crate::warrior::Warrior::pivot)
///
/// **Level 8**
///
/// New abilities (and *directional* counterparts) unlocked at this level:
///
/// * [`look`](crate::warrior::Warrior::look) -> [`look_toward`](crate::warrior::Warrior::look_toward)
/// * [`shoot`](crate::warrior::Warrior::shoot) -> [`shoot_toward`](crate::warrior::Warrior::shoot_toward)
///
/// **Level 9**
///
/// *No new abilities unlocked at this level!*
pub struct Warrior {
    level: usize,
    ahead: Vec<Tile>,
    behind: Vec<Tile>,
    health: i32,
    facing: Direction,
    action: RefCell<Option<Action>>,
}

impl Warrior {
    pub fn new(
        level: usize,
        ahead: Vec<Tile>,
        behind: Vec<Tile>,
        health: i32,
        facing: Direction,
    ) -> Warrior {
        Warrior {
            level,
            ahead,
            behind,
            health,
            facing,
            action: RefCell::new(None),
        }
    }

    /// Walk forward one tile.
    /// This is an [`Action`](crate::actions::Action).
    /// This ability is available at **Level 1**.
    pub fn walk(&self) {
        self.perform_walk(Direction::Forward);
    }

    /// Walk one tile toward specified `direction`.
    /// This is an [`Action`](crate::actions::Action).
    /// This ability is unlocked at **Level 6**.
    pub fn walk_toward(&self, direction: Direction) {
        if self.level < 6 {
            panic!("You have not yet learned `walk_toward`! Perhaps you meant `walk`?")
        }
        self.perform_walk(direction);
    }

    // private helper for `walk` and `walk_toward`
    fn perform_walk(&self, direction: Direction) {
        self.perform(Action::Walk(direction));
    }

    /// Check the tile in front of the Warrior.
    /// Returns a [`Tile`](crate::Tile).
    /// This ability is unlocked at **Level 2**.
    pub fn check(&self) -> Tile {
        if self.level < 2 {
            panic!("You have not yet learned `check`!");
        }
        self.perform_check(Direction::Forward)
    }

    /// Check the tile toward specified `direction`.
    /// Returns a [`Tile`](crate::Tile).
    /// This ability is unlocked at **Level 6**.
    pub fn check_toward(&self, direction: Direction) -> Tile {
        if self.level < 6 {
            panic!("You have not yet learned `check_toward`! Perhaps you meant `check`?")
        }
        self.perform_check(direction)
    }

    // private helper for `check` and `check_toward`
    fn perform_check(&self, direction: Direction) -> Tile {
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
    /// This ability is unlocked at **Level 8**.
    pub fn look(&self) -> &Vec<Tile> {
        if self.level < 8 {
            panic!("You have not yet learned `look`!")
        }
        self.look_toward(Direction::Forward)
    }

    /// Check three tiles toward specified `direction`.
    /// Returns a vector of up to three [`Tile`](crate::Tile)s.
    /// This ability is unlocked at **Level 8**.
    pub fn look_toward(&self, direction: Direction) -> &Vec<Tile> {
        if self.level < 8 {
            panic!("You have not yet learned `look_toward`!")
        }
        match direction {
            Direction::Forward => &self.ahead,
            Direction::Backward => &self.behind,
        }
    }

    /// Attempt to attack an enemy in the tile in front of the Warrior.
    /// This is an [`Action`](crate::actions::Action).
    /// This ability is unlocked at **Level 2**.
    pub fn attack(&self) {
        if self.level < 2 {
            panic!("You have not yet learned `attack`!");
        }
        self.perform_attack(Direction::Forward);
    }

    /// Attempt to attack an enemy one tile away in specified `direction`.
    /// This is an [`Action`](crate::actions::Action).
    /// This ability is unlocked at **Level 6**.
    pub fn attack_toward(&self, direction: Direction) {
        if self.level < 6 {
            panic!("You have not yet learned `attack_toward`! Perhaps you meant `attack`?")
        }
        self.perform_attack(direction);
    }

    // private helper for `attack` and `attack_toward`
    fn perform_attack(&self, direction: Direction) {
        self.perform(Action::Attack(direction));
    }

    /// Check the current health of the Warrior.
    /// This ability is unlocked at **Level 3**.
    pub fn health(&self) -> i32 {
        if self.level < 3 {
            panic!("You have not yet learned `health`!");
        }
        self.health
    }

    /// Rest and regain 10% of the Warrior's HP.
    /// This is an [`Action`](crate::actions::Action).
    /// This ability is unlocked at **Level 3**.
    pub fn rest(&self) {
        if self.level < 3 {
            panic!("You have not yet learned `rest`!");
        }
        self.perform(Action::Rest);
    }

    /// Attempt to rescue a Captive in front of the Warrior.
    /// This is an [`Action`](crate::actions::Action).
    /// This ability is unlocked at **Level 5**.
    pub fn rescue(&self) {
        if self.level < 5 {
            panic!("You have not yet learned `rescue`!");
        }
        self.perform_rescue(Direction::Forward);
    }

    /// Attempt to rescue a Captive one tile away in specified `direction`.
    /// This is an [`Action`](crate::actions::Action).
    /// This ability is unlocked at **Level 6**.
    pub fn rescue_toward(&self, direction: Direction) {
        if self.level < 6 {
            panic!("You have not yet learned `rescue_toward`! Perhaps you meant `rescue`?")
        }
        self.perform_rescue(direction);
    }

    // private helper for `rescue` and `rescue_toward`
    fn perform_rescue(&self, direction: Direction) {
        self.perform(Action::Rescue(direction));
    }

    /// Rotate 180 degrees.
    /// This is an [`Action`](crate::actions::Action).
    /// This ability is unlocked at **Level 7**.
    pub fn pivot(&self) {
        if self.level < 7 {
            panic!("You have not yet learned `pivot`!")
        }
        let direction = match self.facing {
            Direction::Forward => Direction::Backward,
            Direction::Backward => Direction::Forward,
        };
        self.perform(Action::Pivot(direction));
    }

    /// Fire an arrow up to three tiles in front of the Warrior.
    /// This is an [`Action`](crate::actions::Action).
    /// This ability is unlocked at **Level 8**.
    pub fn shoot(&self) {
        if self.level < 8 {
            panic!("You have not yet learned `shoot`!")
        }
        self.shoot_toward(Direction::Forward);
    }

    /// Fire an arrow up to three tiles toward specified `direction`.
    /// This is an [`Action`](crate::actions::Action).
    /// This ability is unlocked at **Level 8**.
    pub fn shoot_toward(&self, direction: Direction) {
        if self.level < 8 {
            panic!("You have not yet learned `shoot_toward`!")
        }
        self.perform(Action::Shoot(direction));
    }

    /// Some [`Action`](crate::actions::Action) the Warrior has performed;
    /// None if no action has been performed.
    pub fn action(&self) -> Option<Action> {
        *self.action.borrow()
    }

    fn perform(&self, action: Action) {
        if self.action.borrow().is_some() {
            println!("Warrior already performed action!");
            return;
        }

        *self.action.borrow_mut() = Some(action);
    }
}
