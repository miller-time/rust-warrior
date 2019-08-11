//! contains system for player-controlled interactions

use std::cmp;

use specs::{prelude::*, System};

use crate::{
    actions::{Action, Direction},
    engine::components::UnitComponent,
    floor::Tile,
    unit::UnitType,
    Player, Warrior,
};

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
        let mut warrior_comp = None;
        let mut other_units = Vec::new();
        for (entity, comp) in (&entities, &mut units).join() {
            match comp.unit.unit_type {
                UnitType::Warrior => {
                    warrior_comp = Some(comp);
                }
                _ => {
                    other_units.push((entity, comp));
                }
            }
        }
        let warrior_comp = warrior_comp.unwrap();
        let (wx, _) = warrior_comp.unit.position;
        let unit_in_range = {
            other_units.iter_mut().find(|(_, comp)| {
                let (sx, _) = comp.unit.position;
                // NOTE: the implied assumption is that there can never be a
                //       unit one space away in both directions,
                //       which so far is always true!
                (wx - sx).abs() == 1
            })
        };
        let (ahead, behind) = match unit_in_range {
            Some((_, comp)) => {
                let (x, _) = comp.unit.position;
                if wx < x {
                    let behind = if wx == 0 { Tile::Wall } else { Tile::Empty };
                    // there is a unit ahead, possibly a wall behind
                    (Tile::Unit(comp.unit.unit_type), behind)
                } else {
                    // there is a unit behind, assume clear ahead
                    (Tile::Empty, Tile::Unit(comp.unit.unit_type))
                }
            }
            None => {
                let behind = if wx == 0 { Tile::Wall } else { Tile::Empty };
                // clear ahead, possibly a wall behind
                (Tile::Empty, behind)
            }
        };
        let (health, _) = warrior_comp.unit.hp;
        let facing = warrior_comp.unit.facing.unwrap();
        let mut warrior = Warrior::new(ahead, behind, health, facing);
        self.player.play_turn(&mut warrior);

        if let Some(action) = warrior.action {
            match action {
                Action::Walk(direction) => {
                    let (x, y) = warrior_comp.unit.position;
                    let (path_clear, new_x) = match direction {
                        Direction::Forward => (ahead == Tile::Empty, x + 1),
                        Direction::Backward => (behind == Tile::Empty, x - 1),
                    };
                    if path_clear {
                        println!(
                            "{warrior} walks {direction:?}",
                            warrior = &self.name,
                            direction = direction
                        );
                        warrior_comp.unit.position = (new_x, y);
                    } else {
                        let (_, enemy_comp) = unit_in_range.unwrap();
                        println!(
                            "{warrior} bumps into {enemy:?}",
                            warrior = &self.name,
                            enemy = enemy_comp.unit.unit_type
                        );
                    }
                }
                Action::Attack(direction) => {
                    let path_clear = match direction {
                        Direction::Forward => ahead == Tile::Empty,
                        Direction::Backward => behind == Tile::Empty,
                    };
                    if path_clear {
                        println!(
                            "{warrior} attacks {direction:?} and hits nothing",
                            warrior = &self.name,
                            direction = direction
                        );
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
                Action::Rescue(direction) => {
                    let (path_clear, captive_found) = match direction {
                        Direction::Forward => {
                            (ahead == Tile::Empty, ahead == Tile::Unit(UnitType::Captive))
                        }
                        Direction::Backward => (
                            behind == Tile::Empty,
                            behind == Tile::Unit(UnitType::Captive),
                        ),
                    };
                    if path_clear {
                        println!(
                            "{warrior} leans {direction:?} to rescue someone, but nobody is here",
                            warrior = &self.name,
                            direction = direction
                        );
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
                            "{warrior} leans {direction:?} to rescue {enemy:?}, but it is not a captive!",
                            warrior = &self.name,
                            direction = direction,
                            enemy = enemy_comp.unit.unit_type
                        );
                    }
                }
                Action::Pivot(direction) => {
                    println!(
                        "{warrior} pivots to face {direction:?}",
                        warrior = &self.name,
                        direction = direction
                    );
                    warrior_comp.unit.facing = Some(direction);
                }
            }
        }
    }
}
