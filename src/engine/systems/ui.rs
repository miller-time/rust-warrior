//! contains system which prints out the state of the world

use specs::{prelude::*, System};

use crate::{engine::components::UnitComponent, floor::Floor, unit::Unit};

/// This system simply calls the `draw` method of
/// [`Floor`](crate::floor::Floor) after each turn is executed.
pub struct UiSystem {
    pub floor: Floor,
}

impl UiSystem {
    pub fn new(floor: Floor) -> UiSystem {
        UiSystem { floor }
    }
}

impl<'a> System<'a> for UiSystem {
    type SystemData = ReadStorage<'a, UnitComponent>;

    fn run(&mut self, units: Self::SystemData) {
        self.floor.units = (&units)
            .join()
            .map(|comp| Unit::new(comp.unit.unit_type, comp.unit.position))
            .collect();
        self.floor.draw();
    }
}
