//! the "C" in ECS

use specs::{prelude::*, Component, VecStorage, World};

use crate::unit::Unit;

/// This component is a very thin wrapper, providing a
/// [`Unit`](crate::unit::Unit) that can be added to the game world.
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct UnitComponent {
    pub unit: Unit,
}

impl UnitComponent {
    pub fn new(unit: Unit) -> UnitComponent {
        UnitComponent { unit }
    }

    /// Add `unit` to the game `world`
    pub fn create(world: &mut World, unit: Unit) {
        world.create_entity().with(UnitComponent::new(unit)).build();
    }
}
