use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;
use toml::Value;

pub struct Profile {
    pub name: String,
    pub directory: String,
    pub level: usize,
}

impl Profile {
    pub fn new(name: String) -> Profile {
        let directory = name.to_lowercase().replace(r"[^a-z0-9]+", "-");
        let level = 0;
        Profile {
            name,
            directory,
            level,
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let mut profile = BTreeMap::new();
        profile.insert(String::from("name"), Value::String(self.name.clone()));
        profile.insert(String::from("level"), Value::Integer(self.level as i64));
        let contents = toml::to_string(&profile).unwrap();
        let path = Path::new("rustwarrior")
            .join(&self.directory)
            .join("profile.toml");
        fs::write(path, contents)
    }
}
