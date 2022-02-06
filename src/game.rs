//! where it all starts

use std::env;
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

impl Default for Game {
    fn default() -> Game {
        let profile = load_profile();

        Game { profile }
    }
}

impl Game {
    pub fn new() -> Game {
        Game::default()
    }

    /// The main entry point when playing the game.
    ///
    /// After loading the player profile and initializing the current
    /// level, the game consists of repeatedly calling `play_turn`
    /// on the player's `Player` instance.
    pub fn play(player_generator: fn() -> Box<dyn Player + Send + Sync>) {
        let mut game = Game::new();
        game.start(player_generator);
    }

    fn start(&mut self, player_generator: fn() -> Box<dyn Player + Send + Sync>) {
        if self.profile.challenge_mode {
            if Floor::exists(self.profile.level + 1) == false {
                self.profile.challenge_mode = false;
                starter::write_profile(&self.profile, None);
                println!("CONGRATULATIONS! You have compeleted Challenge mode!");
                return;
            } else {
                self.profile.increment_level();
                starter::write_profile(&self.profile, None);
                println!("Challenge mode active: starting next level...");
            }
        } else if self.profile.maximus_oxidus {
            println!("Now that you have earned the title Maximus Oxidus, you may choose to hone your skills on any level or play challenge mode.");
            if ui::ask("Challenge mode? (play every level back to back)") {
                self.profile.challenge_mode = true;
            }
            let level = ui::select_level();
            self.profile.level = level;
            starter::write_profile(&self.profile, None);
        } else {
            self.level_completed();
        }
        println!("Starting Level {}", self.profile.level);
        let floor = Floor::load(self.profile.level);
        match engine::start(
            self.profile.name.clone(),
            self.profile.level,
            floor,
            player_generator,
        ) {
            Ok(_) => {
                if self.profile.challenge_mode {
                    println!("Success! You have found the stairs.");
                    self.profile.increment_level();
                    starter::write_profile(&self.profile, None);
                    self.start(player_generator);
                } else {
                    self.level_completed();
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    fn level_completed(&mut self) {
        // TODO: tally points
        if self.profile.maximus_oxidus {
            println!("Success! You have found the stairs.");
        } else if Floor::exists(self.profile.level + 1) {
            println!("Success! You have found the stairs.");
            if env::var("NO_PROMPT").is_ok() {
                return;
            }
            if ui::ask("Would you like to continue on to the next level?") {
                self.profile.increment_level();
                starter::write_readme(&self.profile, self.profile.level, None);
                starter::write_profile(&self.profile, None);
                println!("See (updated) README.md for your next instructions.");
            } else {
                // TODO: "Try to earn more points next time."
                println!("Staying on current level.");
            }
        } else {
            println!("CONGRATULATIONS! You have climbed to the top of the tower and have earned the title Maximus Oxidus.");
            self.profile.maximus_oxidus = true;
            starter::write_profile(&self.profile, None);
        }
    }
}

fn load_profile() -> Profile {
    let contents = fs::read_to_string(".profile").expect("error loading .profile");
    Profile::from_toml(&contents)
}
