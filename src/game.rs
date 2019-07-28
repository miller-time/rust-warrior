use std::fs;

use crate::{profile::Profile, Player, Warrior};

pub fn play(player: impl Player) {
    // TODO: epic mode?
    let warrior = Warrior::default();
    let profile = load_profile();
    println!("Starting level {}", profile.level);
    // TODO: load current level/floor
    // TODO: game loop
    player.play_turn(&warrior);
}

fn load_profile() -> Profile {
    let contents = fs::read_to_string("profile.toml").expect("error loading profile.toml");
    Profile::from_toml(&contents)
}
