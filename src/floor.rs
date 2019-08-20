//! contains types that represent the topology of a level

use std::collections::HashMap;

use crate::unit::{Unit, UnitType};

/// The `Floor::tile` method constructs a conceptual representation of the
/// floor using the `Tile` enum.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Wall,
    Empty,
    Stairs,
    Unit(UnitType),
}

impl Tile {
    /// A character (`&str` for convenience) representation of the tile
    pub fn draw(self) -> &'static str {
        match self {
            Tile::Wall => panic!("attempted to draw a wall"),
            Tile::Empty => " ",
            Tile::Stairs => ">",
            Tile::Unit(unit_type) => unit_type.draw(),
        }
    }
}

/// Each level has a `Floor` with a predefined `width` and `height`,
/// `stairs` positioned at the exit, and one or more `units`. There
/// is a player-controlled [`Warrior`](crate::warrior::Warrior) unit
/// for every level.
#[derive(Clone, Debug)]
pub struct Floor {
    /// the east/west count of tiles
    pub width: usize,
    /// the north/south count of tiles
    pub height: usize,
    /// the position (x, y) of the exit
    pub stairs: (i32, i32),
    /// all of the units that the level contains
    pub units: Vec<Unit>,
}

impl Floor {
    /// Returns the predefined configuration for a given `level` number.
    pub fn load(level: usize) -> Floor {
        match Floor::get(level) {
            Some(level) => level,
            None => unimplemented!(),
        }
    }

    /// Returns `true` if a configuration exists for a given `level` number.
    pub fn exists(level: usize) -> bool {
        Floor::get(level).is_some()
    }

    /// Returns a `Tile` representing the current state of a tile
    /// of the floor at `position`.
    pub fn tile(&self, position: (i32, i32)) -> Tile {
        if position == self.stairs {
            return Tile::Stairs;
        }

        let unit_positions: HashMap<(i32, i32), UnitType> = self
            .units
            .iter()
            .map(|u| (u.position, u.unit_type))
            .collect();

        if let Some(unit_type) = unit_positions.get(&position) {
            return Tile::Unit(*unit_type);
        }

        Tile::Empty
    }

    /// Prints a textual representation of the floor and all
    /// of its units.
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
            4 => Some(Floor {
                width: 8,
                height: 1,
                stairs: (7, 0),
                units: vec![
                    Unit::warrior((0, 0)),
                    Unit::thick_sludge((3, 0)),
                    Unit::archer((4, 0)),
                    Unit::thick_sludge((5, 0)),
                ],
            }),
            5 => Some(Floor {
                width: 8,
                height: 1,
                stairs: (7, 0),
                units: vec![
                    Unit::warrior((0, 0)),
                    Unit::captive((2, 0)),
                    Unit::archer((3, 0)),
                    Unit::archer((4, 0)),
                    Unit::thick_sludge((5, 0)),
                    Unit::captive((6, 0)),
                ],
            }),
            6 => Some(Floor {
                width: 9,
                height: 1,
                stairs: (8, 0),
                units: vec![
                    Unit::captive((0, 0)),
                    Unit::warrior((2, 0)),
                    Unit::thick_sludge((4, 0)),
                    Unit::archer((6, 0)),
                    Unit::archer((7, 0)),
                ],
            }),
            7 => Some(Floor {
                width: 6,
                height: 1,
                stairs: (0, 0),
                units: vec![
                    Unit::archer((1, 0)),
                    Unit::thick_sludge((3, 0)),
                    Unit::warrior((5, 0)),
                ],
            }),
            8 => Some(Floor {
                width: 6,
                height: 1,
                stairs: (5, 0),
                units: vec![
                    Unit::warrior((0, 0)),
                    Unit::captive((2, 0)),
                    Unit::wizard((3, 0)),
                    Unit::wizard((4, 0)),
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
