use crate::{engine::WarriorComponent, floor::Floor, Player, Warrior};

use specs::{prelude::*, System};

pub struct PlayerSystem {
    pub player: Box<dyn Player + Send + Sync>,
}

impl<'a> System<'a> for PlayerSystem {
    type SystemData = WriteStorage<'a, WarriorComponent>;

    fn run(&mut self, mut component: Self::SystemData) {
        for component in (&mut component).join() {
            let mut warrior = Warrior::new(component);
            self.player.play_turn(&mut warrior);
        }
    }
}

pub struct UiSystem;

impl<'a> System<'a> for UiSystem {
    type SystemData = ReadStorage<'a, WarriorComponent>;

    fn run(&mut self, component: Self::SystemData) {
        for component in (&component).join() {
            let floor = Floor::new(component.position);
            floor.draw();
        }
    }
}
