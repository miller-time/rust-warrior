//! contains system for archer and wizard enemy AI

use std::cmp;

use specs::{prelude::*, System};

use crate::{engine::components::UnitComponent, unit::UnitType};

/// This system acts as an enemy AI, attacking the player if an archer or
/// wizard exists and can attack the [`Warrior`](crate::warrior::Warrior).
/// The difference from the sludge is that the archer's arrows (and wizard's
/// wand) can reach the warrior up to three spaces away, as long as there is no
/// other enemy in the way.
pub struct ShooterSystem {
    pub name: String,
}

impl ShooterSystem {
    pub fn new(name: String) -> ShooterSystem {
        ShooterSystem { name }
    }
}

impl<'a> System<'a> for ShooterSystem {
    type SystemData = WriteStorage<'a, UnitComponent>;

    fn run(&mut self, mut units: Self::SystemData) {
        let mut warrior_comp = None;
        let mut shooter_comps = Vec::new();
        let mut enemy_comps = Vec::new();
        for unit_comp in (&mut units).join() {
            match unit_comp.unit.unit_type {
                UnitType::Warrior => {
                    warrior_comp = Some(unit_comp);
                }
                UnitType::Archer | UnitType::Wizard => {
                    shooter_comps.push(unit_comp);
                }
                _ => {
                    enemy_comps.push(unit_comp);
                }
            }
        }
        let warrior_comp = warrior_comp.unwrap();

        for shooter_comp in &shooter_comps {
            let (wx, _) = warrior_comp.unit.position;
            let (sx, _) = shooter_comp.unit.position;
            let (hp, _) = shooter_comp.unit.hp;

            let in_range = (sx - wx).abs() < 4;

            let mut obstructions: Vec<&&mut UnitComponent> = enemy_comps
                .iter()
                .filter(|comp| {
                    let (x, _) = comp.unit.position;
                    (wx < x && x < sx) || (wx > x && x > sx)
                })
                .collect();

            obstructions.extend(shooter_comps.iter().filter(|comp| {
                let (x, _) = comp.unit.position;
                (wx < x && x < sx) || (wx > x && x > sx)
            }));

            // if the enemy is killed and the entity is deleted, but `world.maintain()` hasn't
            // been called yet, then we need to see if the enemy is at 0 hp (dead) here.
            if hp > 0 && in_range && obstructions.is_empty() {
                println!(
                    "{shooter:?} attacks {warrior}",
                    shooter = shooter_comp.unit.unit_type,
                    warrior = &self.name
                );
                let (current, max) = warrior_comp.unit.hp;
                let remaining = cmp::max(current - shooter_comp.unit.atk, 0);
                println!(
                    "{warrior} takes {atk} damage, {remaining} HP left",
                    warrior = &self.name,
                    atk = shooter_comp.unit.atk,
                    remaining = remaining
                );
                warrior_comp.unit.hp = (remaining, max);
            }
        }
    }
}
