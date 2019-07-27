use std::fs;
use std::io;
use std::path::Path;
use std::process;

use crate::{level, profile::Profile, ui};

pub fn start() -> io::Result<()> {
    println!("Welcome to Rust Warrior");

    // TODO: path prefix config?
    if Path::new(".profile").exists() {
        // TODO: load profile
        println!("Profile already exists");
        process::exit(0);
    } else {
        make_game_directory()?;
        let profile = choose_profile();

        // TODO: epic mode?
        play(profile)
    }
}

fn choose_profile() -> Profile {
    // TODO: menu to select profile
    // TODO: menu to select difficulty
    let name = ui::request("Enter a name for your warrior: ");
    Profile::new(name)
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

fn play(profile: Profile) -> io::Result<()> {
    // TODO: check loaded profile current level
    // TODO: only prepare here if at level zero
    prepare_next_level(&profile)?;
    // TODO: increment profile's level

    println!(
        "First level has been generated. See the rustwarrior/{}/README for instructions.",
        &profile.directory
    );

    Ok(())
}

fn prepare_next_level(profile: &Profile) -> io::Result<()> {
    level::generate_player_files(profile)
}
