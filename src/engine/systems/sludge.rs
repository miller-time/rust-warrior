use std::cmp;

use specs::{prelude::*, System};

use crate::engine::components::{UnitComponent, UnitType};

pub struct SludgeSystem;

impl<'a> System<'a> for SludgeSystem {
    type SystemData = WriteStorage<'a, UnitComponent>;

    fn run(&mut self, mut units: Self::SystemData) {
        let mut units = (&mut units).join();
        let warrior_comp = units
            .find(|comp| comp.unit_type == UnitType::Warrior)
            .unwrap();
        if let Some(sludge_comp) = units.find(|comp| comp.unit_type == UnitType::Sludge) {
            let (wx, _) = warrior_comp.position;
            let (sx, _) = sludge_comp.position;
            let (hp, _) = sludge_comp.hp;
            // if the Sludge is killed and the entity is deleted, but `world.maintain()` hasn't
            // been called yet, then we need to see if the Sludge is at 0 hp (dead) here.
            if hp > 0 && (wx - sx).abs() <= 1 {
                println!("Sludge attacks Warrior");
                let (current, max) = warrior_comp.hp;
                let remaining = cmp::max(current - sludge_comp.atk, 0);
                println!(
                    "Warrior takes {} damage, {} HP left",
                    sludge_comp.atk, remaining
                );
                warrior_comp.hp = (remaining, max);
            }
        }
    }
}
