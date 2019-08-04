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
        let mut enemies: Vec<(Entity, &mut UnitComponent)> = all_units
            .by_ref()
            .filter(|(_, comp)| comp.unit.unit_type != UnitType::Warrior)
            .collect();
        let combatant = {
            let (wx, _) = warrior_comp.unit.position;
            enemies.iter().find(|(_, comp)| {
                let (sx, _) = comp.unit.position;
                (wx - sx).abs() == 1
            })
        };
        let path_clear = combatant.is_none();
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
                        let (_, enemy_comp) = combatant.unwrap();
                        println!("Warrior bumps into {:?}", enemy_comp.unit.unit_type);
                    }
                }
                Action::Attack => {
                    if path_clear {
                        println!("Warrior attacks and hits nothing");
                    } else {
                        let (wx, _) = warrior_comp.unit.position;
                        // since the path is not clear, we can definitely find a enemy one space away
                        let (enemy_entity, enemy_comp) = enemies
                            .iter_mut()
                            .find(|(_, comp)| {
                                let (sx, _) = comp.unit.position;
                                (wx - sx).abs() == 1
                            })
                            .unwrap();
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
            }
        }
    }
}
