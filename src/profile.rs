use std::collections::BTreeMap;
use toml::Value;

pub struct Profile {
    pub name: String,
    pub directory: String,
    pub level: usize,
}

impl Profile {
    pub fn new(name: String) -> Profile {
        let directory = name.to_lowercase().replace(r"[^a-z0-9]+", "-");
        let level = 1;
        Profile {
            name,
            directory,
            level,
        }
    }

    pub fn to_toml(&self) -> String {
        let mut profile = BTreeMap::new();
        profile.insert(String::from("name"), Value::String(self.name.clone()));
        profile.insert(String::from("level"), Value::Integer(self.level as i64));
        toml::to_string(&profile).unwrap()
    }
}
