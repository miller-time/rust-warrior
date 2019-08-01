//! contains types that represent the topology of a level

use crate::unit::{Unit, UnitType};

/// The `Floor::tile` method constructs a conceptual representation of the
/// floor using the `Tile` enum.
#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Empty,
    Sludge,
    Stairs,
    Warrior,
}

impl Tile {
    /// A character (`&str` for convenience) representation of the tile
    pub fn draw(self) -> &'static str {
        match self {
            Tile::Empty => " ",
            Tile::Sludge => "s",
            Tile::Stairs => ">",
            Tile::Warrior => "@",
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

    /// Extracts the warrior from the `units`
    pub fn warrior(&self) -> &Unit {
        self.units
            .iter()
            .find(|u| u.unit_type == UnitType::Warrior)
            .unwrap()
    }

    /// Returns all of the sludges (if there are any) in `units`.
    /// The `UnitType::Sludge` unit type is introduced in level 2.
    pub fn sludges(&self) -> Vec<&Unit> {
        self.units
            .iter()
            .filter(|u| u.unit_type == UnitType::Sludge)
            .collect()
    }

    /// Returns a `Tile` representing the current state of a tile
    /// of the floor at `position`.
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
