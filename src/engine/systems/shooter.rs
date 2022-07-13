//! contains system for archer and wizard enemy AI

use std::cmp;

use crate::{engine::world::World, unit::UnitType};

/// This system acts as an enemy AI, attacking the player if an archer or
/// wizard exists and can attack the [`Warrior`](crate::warrior::Warrior).
/// The difference from the sludge is that the archer's arrows (and wizard's
/// wand) can reach the warrior up to three spaces away, as long as there is no
/// other enemy in the way.
pub fn shooter_system(world: &mut World) {
    let (wx, _) = world.warrior.position;

    let mut shooters = Vec::new();
    for unit in &world.other_units {
        if unit.unit_type == UnitType::Archer || unit.unit_type == UnitType::Wizard {
            shooters.push(unit.clone());
        }
    }

    for shooter in shooters {
        let (sx, _) = shooter.position;
        let (hp, _) = shooter.hp;

        let in_range = (sx - wx).abs() < 4;

        let mut obstructions = Vec::new();
        for unit in &world.other_units {
            let (x, _) = unit.position;
            if (wx < x && x < sx) || (wx > x && x > sx) {
                obstructions.push(unit.clone());
            }
        }

        if hp > 0 && in_range && obstructions.is_empty() {
            println!(
                "{shooter:?} attacks {warrior}",
                shooter = shooter.unit_type,
                warrior = &world.player_name
            );
            let (current, max) = world.warrior.hp;
            let remaining = cmp::max(current - shooter.atk, 0);
            println!(
                "{warrior} takes {atk} damage, {remaining} HP left",
                warrior = &world.player_name,
                atk = shooter.atk,
                remaining = remaining
            );
            world.warrior.hp = (remaining, max);
        }
    }
}
