//! contains system which prints out the state of the world

use specs::{prelude::*, System};

use crate::{
    engine::components::UnitComponent,
    floor::Floor,
    unit::{Unit, UnitType},
};

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
        let mut unit_comps = (&units).join();
        let mut units = Vec::new();
        let warrior_comp = unit_comps
            .by_ref()
            .find(|comp| comp.unit.unit_type == UnitType::Warrior)
            .unwrap();
        units.push(Unit::warrior(warrior_comp.unit.position));

        for enemy_comp in unit_comps
            .by_ref()
            .filter(|comp| comp.unit.unit_type != UnitType::Warrior)
        {
            units.push(Unit::new(
                enemy_comp.unit.unit_type,
                enemy_comp.unit.position,
            ));
        }

        self.floor.units = units;
        self.floor.draw();
    }
}
