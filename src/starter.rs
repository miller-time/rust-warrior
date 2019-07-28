use std::fs;
use std::io;
use std::path::Path;
use std::process;

use crate::{profile::Profile, ui};

// TODO: add link to Warrior cargo doc
const README: &str = "# Level 1

You see before yourself a long hallway with stairs at the end.
There is nothing in the way.

Tip: Call `warrior.walk()` in the `Player::play_turn` method.

```
 --------
|@      >|
 --------

  > = Stairs
  @ = Russell (20 HP)
```

----------

When you're ready, use `cargo run` to attempt this challenge.
";

fn generate_main_rs(player: &str) -> String {
    format!(
        "use rust_warrior::{{play, Player, Warrior}};

#[derive(Default)]
pub struct {player};

impl Player for {player} {{
    fn play_turn(&self, warrior: &Warrior) {{}}
}}

fn main() {{
    let player = {player}::default();
    play(player);
}}
",
        player = player
    )
}

// TODO: use git repo instead of relative path
const CARGO_TOML: &str = "[package]
name = \"testwarrior\"
version = \"0.1.0\"
edition = \"2018\"

[dependencies]
rust-warrior = { path = \"../..\" }
";

/// Set up a new game directory and player profile
///
/// Creates a `rustwarrior` directory if one does not exist yet.
/// Then creates a player directory for the chosen player name,
/// with the following contents:
///
/// * `src/main.rs`
/// * `Cargo.toml`
/// * `profile.toml`
///
/// From there, the newly generated crate can be used to start level one.
pub fn generate() -> io::Result<()> {
    println!("Welcome to Rust Warrior");

    create_game_directory()?;
    let mut profile = create_profile();
    create_game_files(&mut profile)?;

    println!(
        "Game files have been generated. See rustwarrior/{}/README.md for instructions.",
        &profile.directory
    );

    Ok(())
}

fn create_game_directory() -> io::Result<()> {
    if Path::new("rustwarrior").exists() {
        return Ok(());
    }

    if ui::ask("No rustwarrior directory found. Would you like to create one?") {
        fs::create_dir("rustwarrior")?;
    } else {
        println!("Unable to continue without directory.");
        process::exit(1);
    }

    Ok(())
}

fn create_profile() -> Profile {
    // TODO: menu to select difficulty
    let name = ui::request("Enter a name for your warrior: ");
    Profile::new(name)
}

fn create_game_files(profile: &mut Profile) -> io::Result<()> {
    let player_dir = Path::new("rustwarrior").join(&profile.directory);
    fs::create_dir(&player_dir)?;
    let src_dir = player_dir.join("src");
    fs::create_dir(&src_dir)?;
    let main_rs = src_dir.join("main.rs");
    fs::write(main_rs, generate_main_rs(&profile.name))?;
    let cargo_toml = player_dir.join("Cargo.toml");
    fs::write(cargo_toml, CARGO_TOML)?;
    let profile_toml = player_dir.join("profile.toml");
    fs::write(profile_toml, &profile.to_toml())?;
    let readme = player_dir.join("README.md");
    fs::write(readme, README)
}
