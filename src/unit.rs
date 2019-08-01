//! contains types that represent units that appear in the game

/// Currently there are just the Warrior and Sludges.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitType {
    Warrior,
    Sludge,
}

/// The state of a unit: its `position`, current/max `hp`, and `atk` power.
#[derive(Copy, Clone, Debug)]
pub struct Unit {
    pub unit_type: UnitType,
    pub position: (i32, i32),
    pub hp: (i32, i32),
    pub atk: i32,
}

impl Unit {
    /// Create a unit of type Warrior (20 HP, 5 ATK) at `position`.
    pub fn warrior(position: (i32, i32)) -> Unit {
        Unit {
            unit_type: UnitType::Warrior,
            position,
            hp: (20, 20),
            atk: 5,
        }
    }

    /// Create a unit of type Sludge (12 HP, 3 ATK) at `position`.
    pub fn sludge(position: (i32, i32)) -> Unit {
        Unit {
            unit_type: UnitType::Sludge,
            position,
            hp: (12, 12),
            atk: 3,
        }
    }
}
