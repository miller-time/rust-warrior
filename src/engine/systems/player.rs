//! contains system for player-controlled interactions

use std::cmp;

use specs::{prelude::*, System};

use crate::{actions::Action, engine::components::UnitComponent, unit::UnitType, Player, Warrior};

/// This system defines all of the interactions that are possible for the
/// player-controlled [`Warrior`](crate::warrior::Warrior). The `play_turn`
/// method is called on [`Player`](crate::player::Player), passing a `&mut`
/// warrior whose actions must be specified.
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
        let mut other_units: Vec<(Entity, &mut UnitComponent)> = all_units
            .by_ref()
            .filter(|(_, comp)| comp.unit.unit_type != UnitType::Warrior)
            .collect();
        let unit_in_range = {
            let (wx, _) = warrior_comp.unit.position;
            other_units.iter_mut().find(|(_, comp)| {
                let (sx, _) = comp.unit.position;
                (wx - sx).abs() == 1
            })
        };
        let (path_clear, captive_found) = match unit_in_range {
            Some((_, comp)) => (false, comp.unit.unit_type == UnitType::Captive),
            None => (true, false),
        };
        let (health, _) = warrior_comp.unit.hp;
        let mut warrior = Warrior::new(path_clear, captive_found, health);
        self.player.play_turn(&mut warrior);

        if let Some(action) = warrior.action {
            match action {
                Action::Walk => {
                    if path_clear {
                        println!("Warrior walks forward");
                        let (x, y) = warrior_comp.unit.position;
                        warrior_comp.unit.position = (x + 1, y);
                    } else {
                        let (_, enemy_comp) = unit_in_range.unwrap();
                        println!("Warrior bumps into {:?}", enemy_comp.unit.unit_type);
                    }
                }
                Action::Attack => {
                    if path_clear {
                        println!("Warrior attacks and hits nothing");
                    } else {
                        let (enemy_entity, enemy_comp) = unit_in_range.unwrap();
                        println!("Warrior attacks {:?}", enemy_comp.unit.unit_type);
                        let (current, max) = enemy_comp.unit.hp;
                        let remaining = cmp::max(current - warrior_comp.unit.atk, 0);
                        println!(
                            "{:?} takes {} damage, {} HP left",
                            enemy_comp.unit.unit_type, warrior_comp.unit.atk, remaining
                        );
                        enemy_comp.unit.hp = (remaining, max);

                        if remaining == 0 {
                            println!("{:?} is dead!", enemy_comp.unit.unit_type);
                            entities.delete(*enemy_entity).unwrap();
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
                Action::Rescue => {
                    if path_clear {
                        println!("Warrior tries to rescue someone, but nobody is here");
                    } else if captive_found {
                        let (captive_entity, _) = unit_in_range.unwrap();
                        println!("Warrior frees Captive from their bindings");
                        println!("Captive escapes!");
                        entities.delete(*captive_entity).unwrap();
                    } else {
                        let (_, enemy_comp) = unit_in_range.unwrap();
                        println!(
                            "Warrior tries to rescue {:?}, but it is not a captive!",
                            enemy_comp.unit.unit_type
                        );
                    }
                }
            }
        }
    }
}
