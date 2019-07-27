use std::fs;
use std::io;
use std::path::Path;
use std::process;

use crate::{level, ui};

pub fn start() -> io::Result<()> {
    println!("Welcome to Rust Warrior");

    // TODO: path prefix config?
    if Path::new(".profile").exists() {
        // TODO: load profile
        println!("Profile already exists");
    } else {
        make_game_directory()?;
    }

    // TODO: epic mode?
    play()
}

fn make_game_directory() -> io::Result<()> {
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

fn play() -> io::Result<()> {
    // TODO: practice?
    println!("Play!");

    // TODO: check loaded profile current level
    prepare_next_level()?;
    // TODO: increment profile's level

    // TODO: configure directory name instead of "player"
    println!("First level has been generated. See the rustwarrior/player/README for instructions.");

    Ok(())
}

fn prepare_next_level() -> io::Result<()> {
    level::generate_player_files()
}
