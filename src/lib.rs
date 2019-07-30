//! # rust-warrior
//!
//! There are some significant differences between `rust-warrior` and
//! `ruby-warrior`. Since Ruby is an interpreted language, the player can
//! write a script to control the warrior and then the `rubywarrior`
//! command invokes the game engine which runs that script. In
//! `rust-warrior`, the player will import the game engine as a library.

pub mod actions;
pub mod engine;
pub mod floor;
pub mod game;
pub mod player;
pub mod profile;
pub mod starter;
pub mod ui;
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
