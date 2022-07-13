//! contains system which prints out the state of the world

use crate::{engine::world::World, unit::Unit};

/// This system simply calls the `draw` method of
/// [`Floor`](crate::floor::Floor) after each turn is executed.
pub fn ui_system(world: &mut World) {
    let mut floor = world.floor.clone();

    floor.units = Vec::new();

    let warrior = Unit::new(world.warrior.unit_type, world.warrior.position);
    floor.units.push(warrior);

    for unit in &world.other_units {
        let updated = Unit::new(unit.unit_type, unit.position);
        floor.units.push(updated);
    }

    floor.draw();
}
