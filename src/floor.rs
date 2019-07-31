use crate::unit::{Unit, UnitType};

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

    pub fn sludge(&self) -> Option<&Unit> {
        self.units.iter().find(|u| u.unit_type == UnitType::Sludge)
    }

    pub fn draw(&self) {
        println!(" {}", "-".repeat(self.width));

        let tiles: Vec<&str> = (0..self.width)
            .map(|x| {
                if (x as i32, 0) == self.warrior().position {
                    "@"
                } else if (x as i32, 0) == self.stairs {
                    ">"
                } else {
                    match self.sludge() {
                        Some(sludge) if (x as i32, 0) == sludge.position => "s",
                        _ => " ",
                    }
                }
            })
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
