//! the "S" in ECS
//!
//! There is a [`PlayerSystem`](crate::engine::systems::PlayerSystem), which
//! hooks into the player-controlled [`Warrior`](crate::warrior::Warrior).
//! There is the [`SludgeSystem`](crate::engine::systems::SludgeSystem),
//! which (when a level contains sludges) performs sludge attacks if the
//! player is in range. There is the
//! [`ShooterSystem`](crate::engine::systems::ShooterSystem), which (when a
//! level contains archers or wizards) performs archer and wizard attacks. And
//! lastly there is the [`UiSystem`](crate::engine::systems::UiSystem), which
//! simply draws the overhead map of floor and any units still alive after each
//! turn is executed.

pub mod player;
pub mod shooter;
pub mod sludge;
pub mod ui;

pub use player::PlayerSystem;
pub use shooter::ShooterSystem;
pub use sludge::SludgeSystem;
pub use ui::UiSystem;
