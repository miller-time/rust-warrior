pub struct Profile {
    pub name: String,
    pub directory: String,
}

impl Profile {
    pub fn new(name: String) -> Profile {
        let directory = name.to_lowercase().replace(r"[^a-z0-9]+", "-");
        Profile { name, directory }
    }
}
