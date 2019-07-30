use specs::{prelude::*, System};

use crate::{
    engine::components::{UnitComponent, UnitType},
    floor::Floor,
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
        let mut units = (&units).join();
        let warrior_comp = units
            .find(|comp| comp.unit_type == UnitType::Warrior)
            .unwrap();
        self.floor.warrior = warrior_comp.position;

        if let Some(sludge_comp) = units.find(|comp| comp.unit_type == UnitType::Sludge) {
            self.floor.sludge = Some(sludge_comp.position);
        } else {
            self.floor.sludge = None;
        }

        self.floor.draw();
    }
}
