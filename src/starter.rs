//! contains some methods for generating the game files

use std::fs;
use std::io;
use std::path::Path;
use std::process;

use crate::{profile::Profile, ui};

fn generate_readme(level: usize, player: &str) -> String {
    match level {
        1 => format!(
            "# Level 1

You see before yourself a long hallway with stairs at the end.
There is nothing in the way.

Tip: Call `warrior.walk()` in the `Player::play_turn` method.

```
 --------
|@      >|
 --------

  > = Stairs
  @ = {player} (20 HP)
```

Warrior abilities: https://docs.rs/rust-warrior/latest/rust_warrior/warrior/struct.Warrior.html

----------

When you're ready, use `cargo run` to attempt this challenge.
",
            player = player
        ),
        2 => format!(
            "# Level 2

It is too dark to see anything, but you smell sludge nearby.

Tip: Add a `rust_warrior::Tile` import and then call `warrior.check()`
to see if there is anything in front of you or if it's `Empty`. Call
`warrior.attack()` to fight the sludge.

```
 --------
|@   s  >|
 --------

  > = Stairs
  @ = {player} (20 HP)
  s = Sludge (12 HP)
```

Warrior abilities: https://docs.rs/rust-warrior/latest/rust_warrior/warrior/struct.Warrior.html

----------

When you're ready, use `cargo run` to attempt this challenge.
",
            player = player
        ),
        3 => format!(
            "# Level 3

The air feels thicker than before. There must be a horde of sludge.

Tip: Be careful not to die! Use `warrior.health()` to keep an eye on your health
and `warrior.rest()` to earn 10% of max health back.

```
 ---------
|@ s ss s>|
 ---------

  > = Stairs
  @ = {player} (20 HP)
  s = Sludge (12 HP)
```

Warrior abilities: https://docs.rs/rust-warrior/latest/rust_warrior/warrior/struct.Warrior.html

----------

When you're ready, use `cargo run` to attempt this challenge.
",
            player = player
        ),
        4 => format!(
            "# Level 4

You can hear bow strings being stretched.

Tip: No new abilities this time, but you must be careful not to rest while
taking damage. Add a `health` field to your `Player` struct and compare it on
each turn to see if you're taking damage.

```
 --------
|@  Sa S>|
 --------

  > = Stairs
  @ = {player} (20 HP)
  S = Thick Sludge (18 HP)
  a = Archer (7 HP)
```

Warrior abilities: https://docs.rs/rust-warrior/latest/rust_warrior/warrior/struct.Warrior.html

----------

When you're ready, use `cargo run` to attempt this challenge.
",
            player = player
        ),
        5 => format!(
            "# Level 5

You hear cries for help. Captives must need rescuing.

Tip: Add a `rust_warrior::UnitType` import and call `warrior.check()` to see
if there is a captive (`Tile::Unit(UnitType::Captive)`). Call
`warrior.rescue()` to rescue them. Don't attack captives.

```
 --------
|@ CaaSC>|
 --------

  > = Stairs
  @ = {player} (20 HP)
  C = Captive (1 HP)
  a = Archer (7 HP)
  S = Thick Sludge (18 HP)
```

Warrior abilities: https://docs.rs/rust-warrior/latest/rust_warrior/warrior/struct.Warrior.html

----------

When you're ready, use `cargo run` to attempt this challenge.
",
            player = player
        ),
        6 => format!(
            "# Level 6

The wall behind you feels a bit further away in this room. And you hear more
cries for help.

Tip: Add a `rust_warrior::Direction` import and use directional actions.
The directional action methods are `warrior.walk_toward(Direction)`,
`warrior.check_toward(Direction)`, `warrior.attack_toward(Direction)`,
and `warrior.rescue_toward(Direction)`. Archer attacks have a limited range.
Walk backward if you are taking damage from afar and don't have enough health
to engage. Consider backing up until you've reached a `Tile::Wall`.

```
 ---------
|C @ S aa>|
 ---------

  > = Stairs
  @ = {player} (20 HP)
  C = Captive (1 HP)
  S = Thick Sludge (18 HP)
  a = Archer (7 HP)
```

Warrior abilities: https://docs.rs/rust-warrior/latest/rust_warrior/warrior/struct.Warrior.html

----------

When you're ready, use `cargo run` to attempt this challenge.
",
            player = player
        ),
        7 => format!(
            "# Level 7

You feel a wall right in front of you and an opening behind you.

Tip: You are not as effective at attacking backward. Use
`warrior.check_toward(Drection)` to see if you are facing a `Tile::Wall` and
`warrior.pivot()` to turn around.

```
 ------
|>a S @|
 ------

  > = Stairs
  @ = {player} (20 HP)
  a = Archer (7 HP)
  S = Thick Sludge (18 HP)
```

Warrior abilities: https://docs.rs/rust-warrior/latest/rust_warrior/warrior/struct.Warrior.html

----------

When you're ready, use `cargo run` to attempt this challenge.
",
            player = player
        ),
        8 => format!(
            "# Level 8

You hear the mumbling of wizards. Beware of their deadly wands! Good thing you
found a bow and some arrows.

Tip: Use `warrior.look` to determine your surroundings, and `warrior.shoot` to
fire an arrow. Wizards are deadly but low in health. Kill them before they
have time to attack.

```
 ------
|@ Cww>|
 ------

  > = Stairs
  @ = {player} (20 HP)
  C = Captive (1 HP)
  w = Wizard (3 HP)
```

Warrior abilities: https://docs.rs/rust-warrior/latest/rust_warrior/warrior/struct.Warrior.html

----------

When you're ready, use `cargo run` to attempt this challenge.
",
            player = player
        ),
        _ => unimplemented!(),
    }
}

fn generate_main_rs(player: &str) -> String {
    format!(
        "use rust_warrior::{{Game, Player, Warrior}};

#[derive(Default)]
pub struct {player};

impl Player for {player} {{
    fn play_turn(&mut self, warrior: &mut Warrior) {{}}
}}

fn main() {{
    let player = {player}::default();
    Game::play(player);
}}
",
        player = player
    )
}

fn generate_cargo_toml(name: &str) -> String {
    format!(
        "[package]
name = \"rustwarrior-{name}\"
version = \"0.1.0\"
edition = \"2018\"

[dependencies]
rust-warrior = \"0.8.0\"
",
        name = name
    )
}

/// Set up a new game directory and player profile
///
/// Creates a `rustwarrior` directory if one does not exist yet.
/// Then creates a player directory for the chosen player name,
/// with the following contents:
///
/// * `src/main.rs`
/// * `Cargo.toml`
/// * `.profile`
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

/// Write the README.md for the current level into the player's game directory
pub fn write_readme(profile: &Profile, directory: Option<&Path>) {
    let readme = if let Some(player_dir) = directory {
        player_dir.join("README.md")
    } else {
        Path::new("README.md").to_path_buf()
    };
    let contents = generate_readme(profile.level, &profile.name);
    fs::write(readme, contents)
        .unwrap_or_else(|_| panic!("failed to generate level {} README.md", profile.level));
}

/// Save the player's [`Profile`](crate::profile::Profile) to .profile in their
/// game directory
pub fn write_profile(profile: &Profile, directory: Option<&Path>) {
    let profile_toml = if let Some(player_dir) = directory {
        player_dir.join(".profile")
    } else {
        Path::new(".profile").to_path_buf()
    };
    fs::write(profile_toml, &profile.to_toml()).expect("failed to write .profile");
}

fn create_game_files(profile: &mut Profile) -> io::Result<()> {
    let player_dir = Path::new("rustwarrior").join(&profile.directory);
    fs::create_dir(&player_dir)?;
    let src_dir = player_dir.join("src");
    fs::create_dir(&src_dir)?;
    let main_rs = src_dir.join("main.rs");
    fs::write(main_rs, generate_main_rs(&profile.name))?;
    let cargo_toml = player_dir.join("Cargo.toml");
    fs::write(cargo_toml, generate_cargo_toml(&profile.directory))?;

    write_profile(profile, Some(&player_dir));
    write_readme(&profile, Some(&player_dir));

    Ok(())
}
