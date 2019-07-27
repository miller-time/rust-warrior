use crate::Warrior;

pub trait Player {
    fn play_turn(&self, warrior: &Warrior);
}
