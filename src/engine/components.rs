use specs::{prelude::*, Component, VecStorage, World};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct WarriorComponent {
    pub position: (i32, i32),
}

impl WarriorComponent {
    pub fn create(world: &mut World) {
        world
            .create_entity()
            .with(WarriorComponent { position: (0, 0) })
            .build();
    }
}
