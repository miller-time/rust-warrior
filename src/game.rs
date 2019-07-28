use std::fs;
use std::{thread, time};

use crate::{level::Level, profile::Profile, Player, Warrior};

/// The main entry point when playing the game.
///
/// After loading the player profile and initializing the current
/// level, the game consists of repeatedly calling `play_turn`
/// on the player's `Player` instance.
pub fn play(player: impl Player) {
    // TODO: epic mode?
    let profile = load_profile();
    let level = Level::new(profile.level);
    let mut warrior = Warrior::new(level);
    loop {
        println!("{}", warrior.level);

        if warrior.level.is_complete() {
            println!("You did it!");
            break;
        }

        player.play_turn(&mut warrior);

        thread::sleep(time::Duration::from_millis(500));
    }
}

fn load_profile() -> Profile {
    let contents = fs::read_to_string(".profile").expect("error loading .profile");
    Profile::from_toml(&contents)
}
