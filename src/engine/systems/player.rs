//! contains system for player-controlled interactions

use std::cmp;

use specs::{prelude::*, System};

use crate::{
    actions::{Action, Direction},
    engine::components::UnitComponent,
    floor::{Floor, Tile},
    unit::UnitType,
    Player, Warrior,
};

/// This system defines all of the interactions that are possible for the
/// player-controlled [`Warrior`](crate::warrior::Warrior). The `play_turn`
/// method is called on [`Player`](crate::player::Player), passing a `&mut`
/// warrior whose actions must be specified.
pub struct PlayerSystem {
    pub name: String,
    pub floor: Floor,
    pub player: Box<dyn Player + Send + Sync>,
}

impl PlayerSystem {
    pub fn new(
        name: String,
        floor: Floor,
        player: impl Player + Send + Sync + 'static,
    ) -> PlayerSystem {
        PlayerSystem {
            name,
            floor,
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
        let (wx, wy) = warrior_comp.unit.position;
        let (health, _) = warrior_comp.unit.hp;
        let facing = warrior_comp.unit.facing.unwrap();

        // NOTE: with the bow this range is 3 spaces
        // TODO: conditionally determine warrior's range
        let x_min = cmp::max(wx - 3, 0);
        let x_max = cmp::min(wx + 3, self.floor.width as i32 - 1);

        let west: Vec<(i32, Tile)> = (x_min..wx)
            .rev()
            .map(|i| {
                let unit = other_units
                    .iter_mut()
                    .map(|(_, comp)| &comp.unit)
                    .find(|unit| {
                        let (x, _) = unit.position;
                        x == i
                    });
                match unit {
                    Some(unit) => (i, Tile::Unit(unit.unit_type)),
                    _ => (i, Tile::Empty),
                }
            })
            .collect();

        let east: Vec<(i32, Tile)> = ((wx + 1)..=x_max)
            .map(|i| {
                let unit = other_units
                    .iter_mut()
                    .map(|(_, comp)| &comp.unit)
                    .find(|unit| {
                        let (x, _) = unit.position;
                        x == i
                    });
                match unit {
                    Some(unit) => (i, Tile::Unit(unit.unit_type)),
                    _ => (i, Tile::Empty),
                }
            })
            .collect();

        let (ahead, behind) = match facing {
            Direction::Forward => (east, west),
            Direction::Backward => (west, east),
        };
        let warrior = Warrior::new(
            self.floor.level,
            // `Vec<(i32, Tile)>` -> `Vec<Tile>`
            ahead.clone().into_iter().map(|(_, t)| t).collect(),
            // `Vec<(i32, Tile)>` -> `Vec<Tile>`
            behind.clone().into_iter().map(|(_, t)| t).collect(),
            health,
            facing,
        );
        self.player.play_turn(&warrior);

        if let Some(action) = warrior.action() {
            match action {
                Action::Walk(direction) => {
                    let target_x = if facing == direction {
                        // either facing Forward and walking Forward
                        // or facing Backward and walking Backward
                        wx + 1
                    } else {
                        // either facing Forward and walking Backward
                        // or facing Backward and walking Forward
                        wx - 1
                    };

                    let other_unit = other_units.iter().find(|(_, comp)| {
                        let (x, _) = comp.unit.position;
                        x == target_x
                    });

                    match other_unit {
                        Some((_, enemy_comp)) => {
                            println!(
                                "{warrior} bumps into {enemy:?}",
                                warrior = &self.name,
                                enemy = enemy_comp.unit.unit_type
                            );
                        }
                        _ => {
                            println!(
                                "{warrior} walks {direction:?}",
                                warrior = &self.name,
                                direction = direction
                            );
                            warrior_comp.unit.position = (target_x, wy);
                        }
                    }
                }
                Action::Attack(direction) => {
                    let target_x = if facing == direction {
                        // either facing Forward and attacking Forward
                        // or facing Backward and attacking Backward
                        wx + 1
                    } else {
                        // either facing Forward and attacking Backward
                        // or facing Backward and attacking Forward
                        wx - 1
                    };

                    let other_unit = other_units.iter_mut().find(|(_, comp)| {
                        let (x, _) = comp.unit.position;
                        x == target_x
                    });

                    match other_unit {
                        Some((enemy_entity, enemy_comp)) => {
                            println!(
                                "{warrior} attacks {direction:?} and hits {enemy:?}",
                                warrior = &self.name,
                                direction = direction,
                                enemy = enemy_comp.unit.unit_type
                            );
                            let atk = match direction {
                                Direction::Forward => warrior_comp.unit.atk,
                                Direction::Backward => {
                                    (warrior_comp.unit.atk as f32 / 2.0).ceil() as i32
                                }
                            };
                            let (current, max) = enemy_comp.unit.hp;
                            let remaining = cmp::max(current - atk, 0);
                            println!(
                                "{enemy:?} takes {atk} damage, {remaining} HP left",
                                enemy = enemy_comp.unit.unit_type,
                                atk = atk,
                                remaining = remaining
                            );
                            enemy_comp.unit.hp = (remaining, max);

                            if remaining == 0 {
                                println!("{:?} is dead!", enemy_comp.unit.unit_type);
                                entities.delete(*enemy_entity).unwrap();
                            }
                        }
                        _ => {
                            println!(
                                "{warrior} attacks {direction:?} and hits nothing",
                                warrior = &self.name,
                                direction = direction
                            );
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
                            "{warrior} regains {restored} HP from resting! Now {remaining} HP left",
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
                    let target_x = if facing == direction {
                        // either facing Forward and rescuing Forward
                        // or facing Backward and rescuing Backward
                        wx + 1
                    } else {
                        // either facing Forward and rescuing Backward
                        // or facing Backward and rescuing Forward
                        wx - 1
                    };

                    let other_unit = other_units.iter().find(|(_, comp)| {
                        let (x, _) = comp.unit.position;
                        x == target_x
                    });

                    match other_unit {
                        Some((captive_entity, captive_comp))
                            if captive_comp.unit.unit_type == UnitType::Captive =>
                        {
                            println!(
                                "{warrior} frees {captive:?} from their bindings",
                                warrior = &self.name,
                                captive = captive_comp.unit.unit_type
                            );
                            println!("{:?} escapes!", captive_comp.unit.unit_type);
                            entities.delete(*captive_entity).unwrap();
                        }
                        Some((_, enemy_comp)) => {
                            println!(
                                "{warrior} leans {direction:?} to rescue {enemy:?}, but it is not a captive!",
                                warrior = &self.name,
                                direction = direction,
                                enemy = enemy_comp.unit.unit_type
                            );
                        }
                        None => {
                            println!(
                                "{warrior} leans {direction:?} to rescue someone, but nobody is here",
                                warrior = &self.name,
                                direction = direction
                            );
                        }
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
                Action::Shoot(direction) => {
                    // find the first unit in the direction the Warrior is shooting, if one exists
                    let target = other_units.iter_mut().find(|(_, comp)| {
                        let (x, _) = comp.unit.position;
                        match direction {
                            Direction::Forward => {
                                match ahead.iter().find(|(_, tile)| *tile != Tile::Empty) {
                                    Some((target_x, _)) if *target_x == x => true,
                                    _ => false,
                                }
                            }
                            Direction::Backward => {
                                match behind.iter().find(|(_, tile)| *tile != Tile::Empty) {
                                    Some((target_x, _)) if *target_x == x => true,
                                    _ => false,
                                }
                            }
                        }
                    });

                    match target {
                        Some((enemy_entity, enemy_comp)) => {
                            println!(
                                "{warrior} lets loose an arrow {direction:?} and hits {enemy:?}",
                                warrior = &self.name,
                                direction = direction,
                                enemy = enemy_comp.unit.unit_type
                            );
                            let atk = (warrior_comp.unit.atk as f32 / 2.0).ceil() as i32;
                            let (current, max) = enemy_comp.unit.hp;
                            let remaining = cmp::max(current - atk, 0);
                            println!(
                                "{enemy:?} takes {atk} damage, {remaining} HP left",
                                enemy = enemy_comp.unit.unit_type,
                                atk = atk,
                                remaining = remaining
                            );
                            enemy_comp.unit.hp = (remaining, max);

                            if remaining == 0 {
                                println!("{:?} is dead!", enemy_comp.unit.unit_type);
                                entities.delete(*enemy_entity).unwrap();
                            }
                        }
                        _ => {
                            println!(
                                "{warrior} lets loose an arrow {direction:?} and hits nothing",
                                warrior = &self.name,
                                direction = direction
                            );
                        }
                    }
                }
            }
        }
    }
}
