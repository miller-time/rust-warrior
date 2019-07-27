use crate::{Player, Warrior};

pub fn play(player: impl Player) {
    // TODO: epic mode?
    let warrior = Warrior::default();
    player.play_turn(&warrior);
}
