use specs::{prelude::*, System};

use crate::{
    engine::components::UnitComponent,
    floor::Floor,
    unit::{Unit, UnitType},
};

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
            .find(|comp| comp.unit.unit_type == UnitType::Warrior)
            .unwrap();
        units.push(Unit::warrior(warrior_comp.unit.position));

        if let Some(sludge_comp) = unit_comps.find(|comp| comp.unit.unit_type == UnitType::Sludge) {
            units.push(Unit::sludge(sludge_comp.unit.position));
        }

        self.floor.units = units;
        self.floor.draw();
    }
}
