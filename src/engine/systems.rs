//! Game engine systems
//!
//! The [`player_system`](crate::engine::systems::player_system) function allows
//! the player to control their [`Warrior`](crate::warrior::Warrior) and
//! responds to their chosen action each turn.
//!
//! If a level contains sludges, then the
//! [`sludge_system`](crate::engine::systems::sludge_system) performs sludge
//! attacks any time a sludge is within range of the player.
//!
//! If a level contains archers or wizards, then the
//! [`shooter_system`](crate::engine::systems::shooter_system) performs archer
//! and wizard attacks.
//!
//! Lastly, the [`ui_system`](crate::engine::systems::ui_system) draws an
//! overhead map of the floor and any units still alive after each turn takes
//! place.

pub mod player;
pub mod shooter;
pub mod sludge;
pub mod ui;

pub use player::player_system;
pub use shooter::shooter_system;
pub use sludge::sludge_system;
pub use ui::ui_system;
