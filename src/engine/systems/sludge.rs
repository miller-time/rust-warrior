//! contains system for sludge enemy AI

use std::cmp;

use crate::{engine::world::World, unit::UnitType};

/// This system acts as an enemy AI, attacking the player if a sludge
/// exists and is in range of the [`Warrior`](crate::warrior::Warrior).
pub fn sludge_system(world: &mut World) {
    let (wx, _) = world.warrior.position;

    let mut sludges = Vec::new();
    for unit in &world.other_units {
        if unit.unit_type == UnitType::Sludge || unit.unit_type == UnitType::ThickSludge {
            sludges.push(unit.clone());
        }
    }

    for sludge in sludges {
        let (sx, _) = sludge.position;
        let (hp, _) = sludge.hp;

        let in_range = (wx - sx).abs() <= 1;

        if hp > 0 && in_range {
            println!(
                "{sludge:?} attacks {warrior}",
                sludge = sludge.unit_type,
                warrior = &world.player_name
            );
            let (current, max) = world.warrior.hp;
            let remaining = cmp::max(current - sludge.atk, 0);
            println!(
                "{warrior} takes {atk} damage, {remaining} HP left",
                warrior = &world.player_name,
                atk = sludge.atk,
                remaining = remaining
            );
            world.warrior.hp = (remaining, max);
        }
    }
}
