//! contains the struct for saving player name and current level

use base64;
use serde_derive::{Deserialize, Serialize};
use std::str;

/// The player profile is essentially just the player's chosen name and
/// the level they are currently working on completing. This is saved in
/// .profile at the root of the player's generated project.
#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub level: usize,
}

impl Profile {
    /// create new Profile for player with given `name`
    pub fn new(name: String) -> Profile {
        let level = 1;
        Profile { name, level }
    }

    pub fn increment_level(&mut self) {
        self.level += 1;
    }

    /// load Profile from base64 encoded TOML String
    pub fn from_toml(contents: &str) -> Profile {
        let err = "failed to parse .profile";
        let bytes = base64::decode(contents).expect(err);
        let decoded = str::from_utf8(&bytes).expect(err);
        toml::from_str(&decoded).expect(err)
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
