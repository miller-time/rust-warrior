use base64;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::str;
use toml::Value;

/// The player profile is the player's name and the current level.
/// This is saved in .profile at the root of the player's
/// project directory.
pub struct Profile {
    pub name: String,
    pub directory: String,
    pub level: usize,
}

impl Profile {
    /// create new Profile for player with given `name`
    pub fn new(name: String) -> Profile {
        let directory = Self::directory_name(&name);
        let level = 1;
        Profile {
            name,
            directory,
            level,
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
        let parsed = decoded.parse::<Value>().expect(err);
        let table = parsed.as_table().expect(err);

        let name = table["name"].as_str().expect(err).to_string();
        let directory = Self::directory_name(&name);
        let level: usize = table["level"]
            .as_integer()
            .expect(err)
            .try_into()
            .expect(err);

        Profile {
            name,
            directory,
            level,
        }
    }

    /// convert Profile to base64 encoded TOML String
    pub fn to_toml(&self) -> String {
        let mut profile = BTreeMap::new();
        profile.insert(String::from("name"), Value::String(self.name.clone()));
        profile.insert(String::from("level"), Value::Integer(self.level as i64));
        let profile_toml = toml::to_string(&profile).unwrap();
        base64::encode(&profile_toml.as_bytes())
    }

    fn directory_name(name: &str) -> String {
        name.to_lowercase().replace(r"[^a-z0-9]+", "-")
    }
}
