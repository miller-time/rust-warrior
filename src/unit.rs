#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitType {
    Warrior,
    Sludge,
}

#[derive(Copy, Clone, Debug)]
pub struct Unit {
    pub unit_type: UnitType,
    pub position: (i32, i32),
    pub hp: (i32, i32),
    pub atk: i32,
}

impl Unit {
    pub fn warrior(position: (i32, i32)) -> Unit {
        Unit {
            unit_type: UnitType::Warrior,
            position,
            hp: (20, 20),
            atk: 5,
        }
    }

    pub fn sludge(position: (i32, i32)) -> Unit {
        Unit {
            unit_type: UnitType::Sludge,
            position,
            hp: (12, 12),
            atk: 3,
        }
    }
}
