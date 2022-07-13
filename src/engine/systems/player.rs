//! contains system for player-controlled interactions

use std::cmp;

use crate::{
    actions::{Action, Direction},
    engine::world::World,
    floor::Tile,
    unit::UnitType,
    Warrior,
};

/// This system defines all of the interactions that are possible for the
/// player-controlled [`Warrior`](crate::warrior::Warrior). The `play_turn`
/// method is called on [`Player`](crate::player::Player), passing a `&mut`
/// warrior whose actions must be specified.
pub fn player_system(world: &mut World) {
    let (wx, wy) = world.warrior.position;
    let (health, _) = world.warrior.hp;
    let facing = world.warrior.facing.unwrap();

    // NOTE: with the bow this range is 3 spaces
    // TODO: conditionally determine warrior's range
    let x_min = cmp::max(wx - 3, 0);
    let x_max = cmp::min(wx + 3, world.floor.width as i32 - 1);

    // include the x value with the tile enum variants
    let west: Vec<(i32, Tile)> = (x_min..wx)
        .rev()
        .map(|i| {
            let unit = world.other_units.iter_mut().find(|unit| {
                let (x, _) = unit.position;
                x == i
            });
            match unit {
                Some(unit) => (i, Tile::Unit(unit.unit_type)),
                _ => (i, Tile::Empty),
            }
        })
        .collect();

    // include the x value with the tile enum variants
    let east: Vec<(i32, Tile)> = ((wx + 1)..=x_max)
        .map(|i| {
            let unit = world.other_units.iter_mut().find(|unit| {
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
        world.warrior_level,
        // `Vec<(i32, Tile)>` -> `Vec<Tile>`
        ahead.clone().into_iter().map(|(_, t)| t).collect(),
        // `Vec<(i32, Tile)>` -> `Vec<Tile>`
        behind.clone().into_iter().map(|(_, t)| t).collect(),
        health,
        facing,
    );

    world.player.play_turn(&warrior);

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

                let other_unit = world.other_units.iter().find(|unit| {
                    let (x, _) = unit.position;
                    x == target_x
                });

                match other_unit {
                    Some(unit) => {
                        println!(
                            "{warrior} bumps into {enemy:?}",
                            warrior = &world.player_name,
                            enemy = unit.unit_type
                        );
                    }
                    _ => {
                        println!(
                            "{warrior} walks {direction:?}",
                            warrior = &world.player_name,
                            direction = direction
                        );
                        world.warrior.position = (target_x, wy);
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

                let other_unit = world.other_units.iter_mut().enumerate().find(|(_, unit)| {
                    let (x, _) = unit.position;
                    x == target_x
                });

                match other_unit {
                    Some((i, enemy)) => {
                        println!(
                            "{warrior} attacks {direction:?} and hits {enemy:?}",
                            warrior = &world.player_name,
                            direction = direction,
                            enemy = enemy.unit_type
                        );
                        let atk = match direction {
                            Direction::Forward => world.warrior.atk,
                            Direction::Backward => (world.warrior.atk as f32 / 2.0).ceil() as i32,
                        };
                        let (current, max) = enemy.hp;
                        let remaining = cmp::max(current - atk, 0);
                        println!(
                            "{enemy:?} takes {atk} damage, {remaining} HP left",
                            enemy = enemy.unit_type,
                            atk = atk,
                            remaining = remaining
                        );
                        enemy.hp = (remaining, max);

                        if remaining == 0 {
                            println!("{:?} is dead!", enemy.unit_type);
                            world.remove_unit(i);
                        }
                    }
                    _ => {
                        println!(
                            "{warrior} attacks {direction:?} and hits nothing",
                            warrior = &world.player_name,
                            direction = direction
                        );
                    }
                }
            }
            Action::Rest => {
                let (current, max) = world.warrior.hp;
                if current < max {
                    let restored = if (current + 2) > max {
                        max - current
                    } else {
                        2
                    };
                    println!(
                        "{warrior} regains {restored} HP from resting! Now {remaining} HP left",
                        warrior = &world.player_name,
                        restored = restored,
                        remaining = current + restored
                    );
                    world.warrior.hp = (current + restored, max);
                } else {
                    println!("{} rests but is already at max HP", &world.player_name);
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

                let other_unit = world.other_units.iter().enumerate().find(|(_, unit)| {
                    let (x, _) = unit.position;
                    x == target_x
                });

                match other_unit {
                    Some((i, captive)) if captive.unit_type == UnitType::Captive => {
                        println!(
                            "{warrior} frees {captive:?} from their bindings",
                            warrior = &world.player_name,
                            captive = captive.unit_type
                        );
                        println!("{:?} escapes!", captive.unit_type);
                        world.remove_unit(i);
                    }
                    Some((_, enemy)) => {
                        println!(
                                "{warrior} leans {direction:?} to rescue {enemy:?}, but it is not a captive!",
                                warrior = &world.player_name,
                                direction = direction,
                                enemy = enemy.unit_type
                            );
                    }
                    None => {
                        println!(
                            "{warrior} leans {direction:?} to rescue someone, but nobody is here",
                            warrior = &world.player_name,
                            direction = direction
                        );
                    }
                }
            }
            Action::Pivot(direction) => {
                println!(
                    "{warrior} pivots to face {direction:?}",
                    warrior = &world.player_name,
                    direction = direction
                );
                world.warrior.facing = Some(direction);
            }
            Action::Shoot(direction) => {
                // find the first unit in the direction the Warrior is shooting, if one exists
                let target = world.other_units.iter_mut().enumerate().find(|(_, unit)| {
                        let (x, _) = unit.position;
                        match direction {
                            Direction::Forward => {
                                matches!(ahead.iter().find(|(_, tile)| *tile != Tile::Empty), Some((target_x, _)) if *target_x == x)
                            }
                            Direction::Backward => {
                                matches!(behind.iter().find(|(_, tile)| *tile != Tile::Empty), Some((target_x, _)) if *target_x == x)
                            }
                        }
                    });

                match target {
                    Some((i, enemy)) => {
                        println!(
                            "{warrior} lets loose an arrow {direction:?} and hits {enemy:?}",
                            warrior = &world.player_name,
                            direction = direction,
                            enemy = enemy.unit_type
                        );
                        let atk = (world.warrior.atk as f32 / 2.0).ceil() as i32;
                        let (current, max) = enemy.hp;
                        let remaining = cmp::max(current - atk, 0);
                        println!(
                            "{enemy:?} takes {atk} damage, {remaining} HP left",
                            enemy = enemy.unit_type,
                            atk = atk,
                            remaining = remaining
                        );
                        enemy.hp = (remaining, max);

                        if remaining == 0 {
                            println!("{:?} is dead!", enemy.unit_type);
                            world.remove_unit(i);
                        }
                    }
                    _ => {
                        println!(
                            "{warrior} lets loose an arrow {direction:?} and hits nothing",
                            warrior = &world.player_name,
                            direction = direction
                        );
                    }
                }
            }
        }
    }
}
