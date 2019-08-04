//! contains types that represent units that appear in the game

/// The Warrior (our protagonist) and enemy Sludges and Archers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitType {
    Warrior,
    Sludge,
    ThickSludge,
    Archer,
}

impl UnitType {
    /// A character (`&str` for convenience) representation of the unit type
    pub fn draw(self) -> &'static str {
        match self {
            UnitType::Warrior => "@",
            UnitType::Sludge => "s",
            UnitType::ThickSludge => "S",
            UnitType::Archer => "a",
        }
    }
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
    /// Create a unit of type `unit_type` at `position`.
    pub fn new(unit_type: UnitType, position: (i32, i32)) -> Unit {
        match unit_type {
            UnitType::Warrior => Unit::warrior(position),
            UnitType::Sludge => Unit::sludge(position),
            UnitType::ThickSludge => Unit::thick_sludge(position),
            UnitType::Archer => Unit::archer(position),
        }
    }

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

    /// Create a unit of type ThickSludge (24 HP, 3 ATK) at `position`.
    pub fn thick_sludge(position: (i32, i32)) -> Unit {
        Unit {
            unit_type: UnitType::ThickSludge,
            position,
            hp: (24, 24),
            atk: 3,
        }
    }

    /// Create a unit of type Archer (7 HP, 3 ATK) at `position`.
    pub fn archer(position: (i32, i32)) -> Unit {
        Unit {
            unit_type: UnitType::Archer,
            position,
            hp: (7, 7),
            atk: 3,
        }
    }
}
