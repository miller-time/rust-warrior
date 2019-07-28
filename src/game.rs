use std::fs;
use std::{thread, time};

use crate::{level::Level, profile::Profile, starter, ui, Player, Warrior};

pub struct Game {
    pub profile: Profile,
    pub warrior: Warrior,
}

impl Game {
    pub fn default() -> Game {
        // TODO: epic mode?
        let profile = load_profile();
        let level = Level::new(profile.level);
        let warrior = Warrior::new(level);

        Game { profile, warrior }
    }

    /// The main entry point when playing the game.
    ///
    /// After loading the player profile and initializing the current
    /// level, the game consists of repeatedly calling `play_turn`
    /// on the player's `Player` instance.
    pub fn play(player: impl Player) {
        let mut game = Game::default();
        game.start(player);
    }

    fn start(&mut self, player: impl Player) {
        println!("Starting Level {}", self.profile.level);
        loop {
            println!("{}", self.warrior.level);

            if self.warrior.level.is_complete() {
                self.level_completed();
                break;
            }

            player.play_turn(&mut self.warrior);

            thread::sleep(time::Duration::from_millis(500));
        }
    }

    fn level_completed(&mut self) {
        // TODO: tally points
        if Level::exists(self.profile.level + 1) {
            println!("Success! You have found the stairs.");
            if ui::ask("Would you like to continue on to the next level?") {
                self.profile.increment_level();
                starter::write_readme(&self.profile, None);
                starter::write_profile(&self.profile, None);
                println!(
                    "See rustwarrior/{}/README.md for your next instructions.",
                    &self.profile.directory
                );
            } else {
                // TODO: "Try to earn more points next time."
                println!("Staying on current level.");
            }
        } else {
            println!("CONGRATULATIONS! You have climbed to the top of the tower and have earned the title Maximus Oxidus.");
        }
    }
}

fn load_profile() -> Profile {
    let contents = fs::read_to_string(".profile").expect("error loading .profile");
    Profile::from_toml(&contents)
}
