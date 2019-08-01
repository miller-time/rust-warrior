//! where it all starts

use std::fs;

use crate::{engine, floor::Floor, profile::Profile, starter, ui, Player};

/// This is exposed to the [`Player`](crate::player::Player) to get things
/// started. Their profile is loaded (from .profile) and then the
/// [`engine`](crate::engine) is fired up. If the current level is
/// completed successfully, then the README.md file and their profile are
/// updated.
pub struct Game {
    pub profile: Profile,
}

impl Game {
    pub fn default() -> Game {
        // TODO: epic mode?
        let profile = load_profile();

        Game { profile }
    }

    /// The main entry point when playing the game.
    ///
    /// After loading the player profile and initializing the current
    /// level, the game consists of repeatedly calling `play_turn`
    /// on the player's `Player` instance.
    pub fn play(player: impl Player + Send + Sync + 'static) {
        let mut game = Game::default();
        game.start(player);
    }

    fn start(&mut self, player: impl Player + Send + Sync + 'static) {
        println!("Starting Level {}", self.profile.level);
        let floor = Floor::load(self.profile.level);
        match engine::start(floor, player) {
            Ok(_) => {
                self.level_completed();
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    fn level_completed(&mut self) {
        // TODO: tally points
        if Floor::exists(self.profile.level + 1) {
            println!("Success! You have found the stairs.");
            if ui::ask("Would you like to continue on to the next level?") {
                self.profile.increment_level();
                starter::write_readme(&self.profile, None);
                starter::write_profile(&self.profile, None);
                println!("See (updated) README.md for your next instructions.");
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
