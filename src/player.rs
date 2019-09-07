//! contains the trait exposed to the player for controlling the Warrior

use crate::Warrior;

/// An implementation of this trait is provided when `rust-warrior`
/// is executed to generate your initial game files. Your struct will
/// be named according to the name you chose.
pub trait Player: Send + Sync {
    /// This method is called by the game engine repeatedly, once per turn.
    /// See [`Warrior`](crate::warrior::Warrior) to see which actions you
    /// can instruct the Warrior to take.
    fn play_turn(&mut self, warrior: &Warrior);
}
