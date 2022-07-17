//! # Rust Warrior
//!
//! A textual turn-based AI game for learning Rust, based on
//! [Ruby Warrior](https://github.com/ryanb/ruby-warrior).
//!
//! You will progress through levels controlling the
//! [`Warrior`](crate::warrior::Warrior), by deciding which action
//! to take based on the surroundings. These decisions will be defined with
//! Rust code in the `play_turn` method of [`Player`](crate::player::Player).
//!
//! ### A Note About Rust vs Ruby
//!
//! There are some notable differences between this game and Ruby
//! Warrior. Since Ruby is an interpreted language, Ruby Warrior is played
//! by interacting with a `player.rb` script which controls the warrior.
//! The `rubywarrior` command then leverages the Ruby interpreter to run
//! that script. In the Rust version, you will generate a Rust project which
//! includes `rust-warrior` as a dependency in its `Cargo.toml` file. The
//! [`Game`](crate::game::Game) is then imported. To run it, you simply use
//! `cargo run` like in any other Rust project.

pub mod actions;
pub mod engine;
pub mod floor;
pub mod game;
pub mod player;
pub mod profile;
pub mod starter;
pub mod ui;
pub mod unit;
pub mod warrior;

pub use actions::Direction;
pub use floor::Tile;
pub use game::Game;
pub use player::Player;
pub use unit::UnitType;
pub use warrior::Warrior;
