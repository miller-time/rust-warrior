use crate::Warrior;

/// As a player of rust-warrior, you will create an implementation of
/// this trait and implement the `play_turn` method, which will be
/// called repeatedly in the main game loop.
pub trait Player {
    fn play_turn(&self, warrior: &mut Warrior);
}
