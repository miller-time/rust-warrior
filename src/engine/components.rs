use specs::{prelude::*, Component, VecStorage, World};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitType {
    Warrior,
    Sludge,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct UnitComponent {
    pub unit_type: UnitType,
    pub position: (i32, i32),
    pub hp: (i32, i32),
    pub atk: i32,
}

impl UnitComponent {
    pub fn create_warrior(world: &mut World, position: (i32, i32)) {
        let warrior = UnitComponent {
            unit_type: UnitType::Warrior,
            position,
            hp: (20, 20),
            atk: 5,
        };
        world.create_entity().with(warrior).build();
    }

    pub fn create_sludge(world: &mut World, position: (i32, i32)) {
        let sludge = UnitComponent {
            unit_type: UnitType::Sludge,
            position,
            hp: (12, 12),
            atk: 3,
        };
        world.create_entity().with(sludge).build();
    }
}
