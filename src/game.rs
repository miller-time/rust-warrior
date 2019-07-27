use std::fs;
use std::io;
use std::path::Path;
use std::process;

use crate::ui::ask;

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
    play();
    Ok(())
}

fn make_game_directory() -> io::Result<()> {
    if Path::new("rustwarrior").exists() {
        return Ok(());
    }

    if ask("No rustwarrior directory found. Would you like to create one?") {
        fs::create_dir("rustwarrior")?;
    } else {
        println!("Unable to continue without directory.");
        process::exit(1);
    }

    Ok(())
}

fn play() {
    println!("Play!");
}
