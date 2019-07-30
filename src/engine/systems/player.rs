use std::cmp;

use specs::{prelude::*, System};

use crate::{
    actions::Action,
    engine::components::{UnitComponent, UnitType},
    Player, Warrior,
};

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
            .find(|(_, comp)| comp.unit_type == UnitType::Warrior)
            .unwrap();
        let (_, mut warrior_comp) = warrior_unit;
        let sludge = all_units
            .by_ref()
            .find(|(_, comp)| comp.unit_type == UnitType::Sludge);
        let path_clear = {
            match &sludge {
                Some((_, sludge_comp)) => {
                    let (wx, _) = warrior_comp.position;
                    let (sx, _) = sludge_comp.position;
                    (wx - sx).abs() > 1
                }
                None => true,
            }
        };
        let mut warrior = Warrior::new(path_clear);
        self.player.play_turn(&mut warrior);

        if let Some(action) = warrior.action {
            match action {
                Action::Walk => {
                    if path_clear {
                        println!("Warrior walks forward");
                        let (x, y) = warrior_comp.position;
                        warrior_comp.position = (x + 1, y);
                    } else {
                        println!("Warrior bumps into Sludge");
                    }
                }
                Action::Attack => {
                    if path_clear {
                        println!("Warrior attacks and hits nothing");
                    } else {
                        match sludge {
                            Some((sludge_entity, sludge_comp)) => {
                                println!("Warrior attacks Sludge");
                                let (current, max) = sludge_comp.hp;
                                let remaining = cmp::max(current - warrior_comp.atk, 0);
                                println!(
                                    "Sludge takes {} damage, {} HP left",
                                    warrior_comp.atk, remaining
                                );
                                sludge_comp.hp = (remaining, max);

                                if remaining == 0 {
                                    println!("Sludge is dead!");
                                    entities.delete(sludge_entity).unwrap();
                                }
                            }
                            None => {
                                println!("Warrior attacks but there is nothing there");
                            }
                        }
                    }
                }
            }
        }
    }
}
