use crate::unit::{Unit, UnitType};

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Empty,
    Sludge,
    Stairs,
    Warrior,
}

impl Tile {
    pub fn draw(self) -> &'static str {
        match self {
            Tile::Empty => " ",
            Tile::Sludge => "s",
            Tile::Stairs => ">",
            Tile::Warrior => "@",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Floor {
    pub width: usize,
    pub height: usize,
    pub stairs: (i32, i32),
    pub units: Vec<Unit>,
}

impl Floor {
    pub fn load(level: usize) -> Floor {
        match Floor::get(level) {
            Some(level) => level,
            None => unimplemented!(),
        }
    }

    pub fn exists(level: usize) -> bool {
        Floor::get(level).is_some()
    }

    pub fn warrior(&self) -> &Unit {
        self.units
            .iter()
            .find(|u| u.unit_type == UnitType::Warrior)
            .unwrap()
    }

    pub fn sludges(&self) -> Vec<&Unit> {
        self.units
            .iter()
            .filter(|u| u.unit_type == UnitType::Sludge)
            .collect()
    }

    pub fn tile(&self, position: (i32, i32)) -> Tile {
        if position == self.stairs {
            return Tile::Stairs;
        }

        if self.warrior().position == position {
            return Tile::Warrior;
        }

        let sludge_positions: Vec<(i32, i32)> = self
            .units
            .iter()
            .filter(|u| u.unit_type == UnitType::Sludge)
            .map(|s| s.position)
            .collect();
        if sludge_positions.contains(&position) {
            Tile::Sludge
        } else {
            Tile::Empty
        }
    }

    pub fn draw(&self) {
        println!(" {}", "-".repeat(self.width));

        let tiles: Vec<&str> = (0..self.width)
            .map(|x| self.tile((x as i32, 0)).draw())
            .collect();
        println!("|{}|", tiles.join(""));

        println!(" {}", "-".repeat(self.width));
    }

    fn get(level: usize) -> Option<Floor> {
        match level {
            1 => Some(Floor {
                units: vec![Unit::warrior((0, 0))],
                ..Floor::default()
            }),
            2 => Some(Floor {
                units: vec![Unit::warrior((0, 0)), Unit::sludge((4, 0))],
                ..Floor::default()
            }),
            3 => Some(Floor {
                width: 9,
                height: 1,
                stairs: (8, 0),
                units: vec![
                    Unit::warrior((0, 0)),
                    Unit::sludge((2, 0)),
                    Unit::sludge((4, 0)),
                    Unit::sludge((5, 0)),
                    Unit::sludge((7, 0)),
                ],
            }),
            _ => None,
        }
    }

    fn default() -> Floor {
        Floor {
            width: 8,
            height: 1,
            stairs: (7, 0),
            units: Vec::new(),
        }
    }
}
