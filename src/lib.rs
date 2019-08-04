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
//!
//! ### The Game Engine
//!
//! The game engine uses the [specs][specs] (**S**pecs **P**arallel **ECS**)
//! crate, which is an [ECS][ecs] library. This might be a familiar tool for
//! game developers. It will hopefully increase the quality of the game's
//! implementation (and possibly allow for an easier transition to an
//! [Amethyst](https://amethyst.rs/) version at some point).
//!
//! [specs]: https://github.com/slide-rs/specs
//! [ecs]: https://en.wikipedia.org/wiki/Entity_component_system

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

pub use game::Game;
pub use player::Player;
pub use warrior::Warrior;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
