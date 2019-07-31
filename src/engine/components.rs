use specs::{prelude::*, Component, VecStorage, World};

use crate::unit::Unit;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct UnitComponent {
    pub unit: Unit,
}

impl UnitComponent {
    pub fn new(unit: Unit) -> UnitComponent {
        UnitComponent { unit }
    }

    pub fn create(world: &mut World, unit: Unit) {
        world.create_entity().with(UnitComponent::new(unit)).build();
    }
}
