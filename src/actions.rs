//! actions the player can instruct the Warrior to take

/// Certain `Action`s are done one tile away, and must be done either
/// while facing forwards or backwards.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Forward
    }
}

/// Certain [`Warrior`](crate::warrior::Warrior) methods correlate to
/// an `Action`. Each turn only one action can be taken. If an action
/// is not successful, then the turn is wasted!
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    /// walk forward one tile
    Walk(Direction),
    /// attack an enemy unit one tile away
    Attack(Direction),
    /// rest to regain 10% HP
    Rest,
    /// rescue a captive one tile away
    Rescue(Direction),
    /// rotate 180 degrees
    Pivot(Direction),
}
