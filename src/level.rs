use std::fs;
use std::io;
use std::path::Path;

use crate::{profile::Profile, templates::starter};

pub fn generate_player_files(profile: &Profile) -> io::Result<()> {
    println!("Generating files...");
    // TODO: check loaded profile's current level, etc
    let player_dir = Path::new("rustwarrior").join(&profile.directory);
    fs::create_dir(&player_dir)?;
    let src_dir = player_dir.join("src");
    fs::create_dir(&src_dir)?;
    let main_rs = src_dir.join("main.rs");
    fs::write(main_rs, starter::MAIN_RS)?;
    let cargo_toml = player_dir.join("Cargo.toml");
    fs::write(cargo_toml, starter::CARGO_TOML)?;
    Ok(())
}
