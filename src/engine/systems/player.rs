//! contains system for player-controlled interactions

use std::cmp;

use specs::{prelude::*, System};

use crate::{actions::Action, engine::components::UnitComponent, unit::UnitType, Player, Warrior};

/// This system defines all of the interactions that are possible for the
/// player-controlled [`Warrior`](crate::warrior::Warrior). The `play_turn`
/// method is called on [`Player`](crate::player::Player), passing a `&mut`
/// warrior whose actions must be specified.
pub struct PlayerSystem {
    pub name: String,
    pub player: Box<dyn Player + Send + Sync>,
}

impl PlayerSystem {
    pub fn new(name: String, player: impl Player + Send + Sync + 'static) -> PlayerSystem {
        PlayerSystem {
            name,
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
                        println!("{} walks forward", &self.name);
                        let (x, y) = warrior_comp.unit.position;
                        warrior_comp.unit.position = (x + 1, y);
                    } else {
                        let (_, enemy_comp) = unit_in_range.unwrap();
                        println!(
                            "{warrior} bumps into {enemy:?}",
                            warrior = &self.name,
                            enemy = enemy_comp.unit.unit_type
                        );
                    }
                }
                Action::Attack => {
                    if path_clear {
                        println!("{} attacks and hits nothing", &self.name);
                    } else {
                        let (enemy_entity, enemy_comp) = unit_in_range.unwrap();
                        println!(
                            "{warrior} attacks {enemy:?}",
                            warrior = &self.name,
                            enemy = enemy_comp.unit.unit_type
                        );
                        let (current, max) = enemy_comp.unit.hp;
                        let remaining = cmp::max(current - warrior_comp.unit.atk, 0);
                        println!(
                            "{enemy:?} takes {atk} damage, {remaining} HP left",
                            enemy = enemy_comp.unit.unit_type,
                            atk = warrior_comp.unit.atk,
                            remaining = remaining
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
                            "{warrior} regains {restored} HP from resting! Now {remaining} left",
                            warrior = &self.name,
                            restored = restored,
                            remaining = current + restored
                        );
                        warrior_comp.unit.hp = (current + restored, max);
                    } else {
                        println!("{} rests but is already at max HP", &self.name);
                    };
                }
                Action::Rescue => {
                    if path_clear {
                        println!("{} tries to rescue someone, but nobody is here", &self.name);
                    } else if captive_found {
                        let (captive_entity, captive_comp) = unit_in_range.unwrap();
                        println!(
                            "{warrior} frees {captive:?} from their bindings",
                            warrior = &self.name,
                            captive = captive_comp.unit.unit_type
                        );
                        println!("{:?} escapes!", captive_comp.unit.unit_type);
                        entities.delete(*captive_entity).unwrap();
                    } else {
                        let (_, enemy_comp) = unit_in_range.unwrap();
                        println!(
                            "{warrior} tries to rescue {enemy:?}, but it is not a captive!",
                            warrior = &self.name,
                            enemy = enemy_comp.unit.unit_type
                        );
                    }
                }
            }
        }
    }
}
