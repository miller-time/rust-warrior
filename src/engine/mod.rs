//! ECS-based game engine
//!
//! This module contains the [specs][specs] implementation, which defines the
//! interactions that occur when levels are played. Unsurprisingly, this
//! [ECS][ecs]-based engine has Entities, Components, and Systems.
//!
//! ### Entities
//!
//! There are one or more entities created, depending on the level. There
//! is always a warrior, there can be one or more sludge or archer enemies,
//! and there can be one or more captives.
//!
//! ### Components
//!
//! See `components` module.
//!
//! ### Systems
//!
//! See `systems` module.
//!
//! [specs]: https://github.com/slide-rs/specs
//! [ecs]: https://en.wikipedia.org/wiki/Entity_component_system

use std::{thread, time};

use specs::{prelude::*, World};

use crate::{floor::Floor, unit::UnitType, Player};

pub mod components;
pub mod systems;

use components::UnitComponent;
use systems::{ArcherSystem, PlayerSystem, SludgeSystem, UiSystem};

/// The entry point for the engine, called by [`Game`](crate::game::Game)
pub fn start(floor: Floor, player: impl Player + Send + Sync + 'static) -> Result<(), String> {
    let mut world = World::new();

    let player_system = PlayerSystem::new(player);
    let ui_system = UiSystem::new(floor.clone());
    let mut dispatcher = DispatcherBuilder::new()
        .with(player_system, "player", &[])
        .with(SludgeSystem, "sludge", &["player"])
        .with(ArcherSystem, "archer", &["player", "sludge"])
        .with(ui_system, "ui", &["player", "sludge", "archer"])
        .build();

    dispatcher.setup(&mut world);

    for unit in &floor.units {
        UnitComponent::create(&mut world, unit.clone());
    }

    floor.draw();

    let mut step = 0;
    loop {
        step += 1;

        {
            let units = world.read_storage::<UnitComponent>();
            for entity in world.entities().join() {
                if let Some(warrior_comp) = units.get(entity) {
                    if let UnitType::Warrior(name) = &warrior_comp.unit.unit_type {
                        let (current, _) = warrior_comp.unit.hp;
                        if current == 0 {
                            return Err(format!("{} died!", name));
                        }
                        if step > 100 {
                            return Err(format!("{} seems to have gotten lost...", name));
                        }
                        if warrior_comp.unit.position == floor.stairs {
                            return Ok(());
                        }
                    }
                }
            }
        }

        dispatcher.dispatch(&world);
        world.maintain();

        thread::sleep(time::Duration::from_millis(500));
    }
}
