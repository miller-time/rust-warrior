//! contains system which prints out the state of the world

#[cfg(feature = "ncurses")]
use crate::engine::curses;

use crate::{engine::world::World, floor::Floor, unit::Unit};

#[cfg(feature = "ncurses")]
pub fn ui_system(world: &World, events: Vec<String>, c: &mut curses::Curses) {
    let floor = update_floor(world);
    c.clear();
    c.println(&floor.draw());
    for e in events {
        c.println(&e);
    }
}

/// This system simply calls the `draw` method of
/// [`Floor`](crate::floor::Floor) after each turn is executed.
#[cfg(not(feature = "ncurses"))]
pub fn ui_system(world: &World, events: Vec<String>) {
    let floor = update_floor(world);
    println!("{}", floor.draw());
    for e in events {
        println!("{}", e);
    }
}

fn update_floor(world: &World) -> Floor {
    let mut floor = world.floor.clone();

    floor.units = Vec::new();

    let warrior = Unit::new(world.warrior.unit_type, world.warrior.position);
    floor.units.push(warrior);

    for unit in &world.other_units {
        let updated = Unit::new(unit.unit_type, unit.position);
        floor.units.push(updated);
    }

    floor
}
