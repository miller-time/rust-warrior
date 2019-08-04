//! the "S" in ECS
//!
//! There is a [`PlayerSystem`](crate::engine::systems::PlayerSystem), which
//! hooks into the player-controlled [`Warrior`](crate::warrior::Warrior).
//! There is the [`SludgeSystem`](crate::engine::systems::SludgeSystem),
//! which (when a level contains sludges) performs sludge attacks if the
//! player is in range. There is the
//! [`ArcherSystem`](crate::engine::systems::ArcherSystem), which (when a
//! level contains archers) performs archer attacks. And lastly there is the
//! [`UiSystem`](crate::engine::systems::UiSystem), which simply draws the
//! overhead map of floor and any units still alive after each turn is
//! executed.

pub mod archer;
pub mod player;
pub mod sludge;
pub mod ui;

pub use archer::ArcherSystem;
pub use player::PlayerSystem;
pub use sludge::SludgeSystem;
pub use ui::UiSystem;
