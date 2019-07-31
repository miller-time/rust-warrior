use std::{thread, time};

use specs::{prelude::*, World};

use crate::{floor::Floor, unit::UnitType, Player};

pub mod components;
pub mod systems;

use components::UnitComponent;
use systems::{PlayerSystem, SludgeSystem, UiSystem};

pub fn start(floor: Floor, player: impl Player + Send + Sync + 'static) -> Result<(), String> {
    let mut world = World::new();

    let player_system = PlayerSystem::new(player);
    let ui_system = UiSystem::new(floor.clone());
    let mut dispatcher = DispatcherBuilder::new()
        .with(player_system, "player", &[])
        .with(SludgeSystem, "sludge", &["player"])
        .with(ui_system, "ui", &["player", "sludge"])
        .build();

    dispatcher.setup(&mut world);

    UnitComponent::create(&mut world, *floor.warrior());

    for sludge in floor.sludges() {
        UnitComponent::create(&mut world, *sludge);
    }

    floor.draw();

    let mut step = 0;
    loop {
        step += 1;

        if step > 20 {
            return Err("You seem to have gotten lost...".to_owned());
        }

        {
            let units = world.read_storage::<UnitComponent>();
            for entity in world.entities().join() {
                match units.get(entity) {
                    Some(warrior_comp) if warrior_comp.unit.unit_type == UnitType::Warrior => {
                        let (current, _) = warrior_comp.unit.hp;
                        if current == 0 {
                            return Err("You died!".to_owned());
                        }
                        if warrior_comp.unit.position == floor.stairs {
                            return Ok(());
                        }
                    }
                    _ => {}
                }
            }
        }

        dispatcher.dispatch(&world);
        world.maintain();

        thread::sleep(time::Duration::from_millis(500));
    }
}
