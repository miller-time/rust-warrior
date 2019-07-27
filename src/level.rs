use std::fs;
use std::io;
use std::path::Path;

use crate::templates::starter;

pub fn generate_player_files() -> io::Result<()> {
    println!("Generating files...");
    // TODO: check loaded profile's current level, etc
    let root = Path::new("rustwarrior");
    let src_dir = root.join("src");
    fs::create_dir(&src_dir)?;
    let main_rs = src_dir.join("main.rs");
    fs::write(main_rs, starter::MAIN_RS)?;
    let cargo_toml = root.join("Cargo.toml");
    fs::write(cargo_toml, starter::CARGO_TOML)?;
    Ok(())
}
