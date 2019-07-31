use std::cmp;

use specs::{prelude::*, System};

use crate::{actions::Action, engine::components::UnitComponent, unit::UnitType, Player, Warrior};

pub struct PlayerSystem {
    pub player: Box<dyn Player + Send + Sync>,
}

impl PlayerSystem {
    pub fn new(player: impl Player + Send + Sync + 'static) -> PlayerSystem {
        PlayerSystem {
            player: Box::new(player),
        }
    }
}

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, UnitComponent>);

    fn run(&mut self, (entities, mut units): Self::SystemData) {
        let mut all_units = (&entities, &mut units).join();
        let warrior_unit = all_units
            .find(|(_, comp)| comp.unit.unit_type == UnitType::Warrior)
            .unwrap();
        let (_, mut warrior_comp) = warrior_unit;
        let mut sludges: Vec<(Entity, &mut UnitComponent)> = all_units
            .by_ref()
            .filter(|(_, comp)| comp.unit.unit_type == UnitType::Sludge)
            .collect();
        let path_clear = {
            let (wx, _) = warrior_comp.unit.position;
            // the path is clear if all sludges are more than a space away
            sludges.iter().all(|(_, comp)| {
                let (sx, _) = comp.unit.position;
                (wx - sx).abs() > 1
            })
        };
        let (health, _) = warrior_comp.unit.hp;
        let mut warrior = Warrior::new(path_clear, health);
        self.player.play_turn(&mut warrior);

        if let Some(action) = warrior.action {
            match action {
                Action::Walk => {
                    if path_clear {
                        println!("Warrior walks forward");
                        let (x, y) = warrior_comp.unit.position;
                        warrior_comp.unit.position = (x + 1, y);
                    } else {
                        println!("Warrior bumps into Sludge");
                    }
                }
                Action::Attack => {
                    if path_clear {
                        println!("Warrior attacks and hits nothing");
                    } else {
                        let (wx, _) = warrior_comp.unit.position;
                        // since the path is not clear, we can definitely find a sludge one space away
                        let (sludge_entity, sludge_comp) = sludges
                            .iter_mut()
                            .find(|(_, comp)| {
                                let (sx, _) = comp.unit.position;
                                (wx - sx).abs() == 1
                            })
                            .unwrap();
                        println!("Warrior attacks Sludge");
                        let (current, max) = sludge_comp.unit.hp;
                        let remaining = cmp::max(current - warrior_comp.unit.atk, 0);
                        println!(
                            "Sludge takes {} damage, {} HP left",
                            warrior_comp.unit.atk, remaining
                        );
                        sludge_comp.unit.hp = (remaining, max);

                        if remaining == 0 {
                            println!("Sludge is dead!");
                            entities.delete(*sludge_entity).unwrap();
                        }
                    }
                }
                Action::Rest => {
                    let (current, max) = warrior_comp.unit.hp;
                    if current < max {
                        let restored = if (current + 2) > max {
                            max - current
                        } else {
                            2
                        };
                        println!(
                            "Warrior regains {} HP from resting! Now {} left",
                            restored,
                            current + restored
                        );
                        warrior_comp.unit.hp = (current + restored, max);
                    } else {
                        println!("Warrior rests but is already at max HP");
                    };
                }
            }
        }
    }
}
