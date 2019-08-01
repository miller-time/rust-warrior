//! actions the player can instruct the Warrior to take

/// Certain [`Warrior`](crate::warrior::Warrior) methods correlate to
/// an `Action`. Each turn only one action can be taken. If an action
/// is not successful, then the turn is wasted!
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    /// walk forward one tile
    Walk,
    /// attack an enemy unit one tile away
    Attack,
    /// rest to regain 10% HP
    Rest,
}
