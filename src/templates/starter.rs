pub const MAIN_RS: &str = "use rust_warrior::Warrior;

fn main() {

}
";

// TODO: use git repo instead of relative path
pub const CARGO_TOML: &str = "[package]
name = \"testwarrior\"
version = \"0.1.0\"
edition = \"2018\"

[dependencies]
rust-warrior = { path = \"../..\" }
";
