//! contains the struct for saving player name and current level

use base64;
use serde_derive::{Deserialize, Serialize};
use std::str;

/// The player's profile tracks their game progress. It is saved in .profile at
/// the root of the player's generated project.
#[derive(Deserialize, Serialize)]
pub struct Profile {
    /// The name the player has chosen
    pub name: String,
    /// The level of the player's warrior
    pub level: usize,
    /// Whether the player has successfully completed the final floor
    pub maximus_oxidus: bool,
    /// If player has chosen to compete in challenge mode
    pub challenge_mode: bool,
}

impl Profile {
    /// create new Profile for player with given `name`
    pub fn new(name: String) -> Profile {
        Profile {
            name,
            level: 1,
            maximus_oxidus: false,
            challenge_mode: false,
        }
    }

    pub fn increment_level(&mut self) {
        self.level += 1;
    }

    /// load Profile from base64 encoded TOML String
    pub fn from_toml(contents: &str) -> Profile {
        let err = "failed to parse .profile";
        let bytes = base64::decode(contents).expect(err);
        let decoded = str::from_utf8(&bytes).expect(err);
        toml::from_str(decoded).expect(err)
    }

    /// convert Profile to base64 encoded TOML String
    pub fn to_toml(&self) -> String {
        let profile_toml = toml::to_string(&self).unwrap();
        base64::encode(&profile_toml.as_bytes())
    }

    pub fn directory(&self) -> String {
        self.name.to_lowercase().replace(r"[^a-z0-9]+", "-")
    }
}
