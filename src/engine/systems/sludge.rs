use std::cmp;

use specs::{prelude::*, System};

use crate::{engine::components::UnitComponent, unit::UnitType};

pub struct SludgeSystem;

impl<'a> System<'a> for SludgeSystem {
    type SystemData = WriteStorage<'a, UnitComponent>;

    fn run(&mut self, mut units: Self::SystemData) {
        let mut units = (&mut units).join();
        let warrior_comp = units
            .find(|comp| comp.unit.unit_type == UnitType::Warrior)
            .unwrap();
        if let Some(sludge_comp) = units.find(|comp| comp.unit.unit_type == UnitType::Sludge) {
            let (wx, _) = warrior_comp.unit.position;
            let (sx, _) = sludge_comp.unit.position;
            let (hp, _) = sludge_comp.unit.hp;
            // if the Sludge is killed and the entity is deleted, but `world.maintain()` hasn't
            // been called yet, then we need to see if the Sludge is at 0 hp (dead) here.
            if hp > 0 && (wx - sx).abs() <= 1 {
                println!("Sludge attacks Warrior");
                let (current, max) = warrior_comp.unit.hp;
                let remaining = cmp::max(current - sludge_comp.unit.atk, 0);
                println!(
                    "Warrior takes {} damage, {} HP left",
                    sludge_comp.unit.atk, remaining
                );
                warrior_comp.unit.hp = (remaining, max);
            }
        }
    }
}
