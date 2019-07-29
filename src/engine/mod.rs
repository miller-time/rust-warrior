use std::{thread, time};

use specs::{prelude::*, World};

use crate::Player;

pub mod components;
pub mod systems;

pub use components::WarriorComponent;
use systems::{PlayerSystem, UiSystem};

pub fn start(player: impl Player + Send + Sync + 'static) -> Result<(), String> {
    let mut world = World::new();
    let player_system = PlayerSystem {
        player: Box::new(player),
    };
    let mut dispatcher = DispatcherBuilder::new()
        .with(player_system, "player", &[])
        .with(UiSystem, "ui", &["player"])
        .build();
    dispatcher.setup(&mut world);
    WarriorComponent::create(&mut world);

    let mut step = 0;
    loop {
        step += 1;

        if step > 20 {
            return Err("You seem to have gotten lost...".to_owned());
        }

        {
            let warriors = world.read_storage::<WarriorComponent>();
            for entity in world.entities().join() {
                if let Some(warrior) = warriors.get(entity) {
                    if warrior.position == (7, 0) {
                        return Ok(());
                    }
                }
            }
        }

        dispatcher.dispatch(&world);
        world.maintain();

        thread::sleep(time::Duration::from_millis(500));
    }
}
