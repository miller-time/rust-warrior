//! The game engine
//!
//! This module was formerly the home of a [specs][specs] implementation but is
//! now home to a zero-dependency version of the engine that functions almost
//! identically. It ended up being a little more straightforward when entities
//! did not need to be queried.
//!
//! When `start` is called, a mutable `World` instance is created. This keeps
//! track of things like the warrior's health and position, plus all enemy
//! units' health and position as well.
//!
//! Within the game's loop, mutable references to the `World` are handed to
//! various systems that live in the `systems` module and define a portion
//! of the game's logic.
//!
//! [specs]: https://github.com/slide-rs/specs

use std::{thread, time};

use crate::{floor::Floor, unit::UnitType, Player};

pub mod systems;
pub mod world;

use systems::{player_system, shooter_system, sludge_system, ui_system};
use world::World;

/// The entry point for the engine, called by [`Game`](crate::game::Game)
pub fn start(
    player_name: String,
    warrior_level: usize,
    floor: Floor,
    player_generator: fn() -> Box<dyn Player + Send + Sync>,
) -> Result<(), String> {
    let player = player_generator();

    println!("{}", floor.draw());

    let mut step = 0;

    let mut warrior = None;
    let mut other_units = Vec::new();
    for unit in &floor.units {
        match unit.unit_type {
            UnitType::Warrior => {
                warrior = Some(unit.clone());
            }
            _ => {
                other_units.push(unit.clone());
            }
        }
    }
    let warrior = warrior.unwrap();

    let mut world = World::new(
        player_name,
        warrior_level,
        floor,
        player,
        warrior,
        other_units,
    );

    loop {
        step += 1;

        if step > 100 {
            return Err(format!(
                "{} seems to have gotten lost...",
                &world.player_name
            ));
        }

        let (current, _) = world.warrior.hp;
        if current == 0 {
            return Err(format!("{} died!", &world.player_name));
        }
        if world.warrior.position == world.floor.stairs {
            return Ok(());
        }

        let mut events = Vec::new();

        let mut player_events = player_system(&mut world);
        events.append(&mut player_events);

        let mut sludge_events = sludge_system(&mut world);
        events.append(&mut sludge_events);

        shooter_system(&mut world);
        ui_system(&mut world);

        thread::sleep(time::Duration::from_millis(500));
    }
}
