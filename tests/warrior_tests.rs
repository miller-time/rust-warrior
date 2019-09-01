use rust_warrior::{actions::Action, Direction, Tile, UnitType, Warrior};

// Actions

#[test]
fn test_walk() {
    let mut warrior = warrior_at_level(1);
    warrior.walk();
    assert_eq!(warrior.action(), Some(Action::Walk(Direction::Forward)));
}

#[test]
fn test_attack() {
    let mut warrior = warrior_at_level(2);
    warrior.attack();
    assert_eq!(warrior.action(), Some(Action::Attack(Direction::Forward)));
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_attack_not_unlocked() {
    let mut warrior = warrior_at_level(1);
    warrior.attack();
}

#[test]
fn test_rest() {
    let mut warrior = warrior_at_level(3);
    warrior.rest();
    assert_eq!(warrior.action(), Some(Action::Rest));
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_rest_not_unlocked() {
    let mut warrior = warrior_at_level(2);
    warrior.rest();
}

#[test]
fn test_rescue() {
    let mut warrior = warrior_at_level(5);
    warrior.rescue();
    assert_eq!(warrior.action(), Some(Action::Rescue(Direction::Forward)));
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_rescue_not_unlocked() {
    let mut warrior = warrior_at_level(4);
    warrior.rescue();
}

#[test]
fn test_pivot() {
    let mut warrior = warrior_at_level(7);
    warrior.pivot();
    assert_eq!(warrior.action(), Some(Action::Pivot(Direction::Backward)));
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_pivot_not_unlocked() {
    let mut warrior = warrior_at_level(6);
    warrior.pivot();
}

#[test]
fn test_shoot() {
    let mut warrior = warrior_at_level(8);
    warrior.shoot();
    assert_eq!(warrior.action(), Some(Action::Shoot(Direction::Forward)));
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_shoot_not_unlocked() {
    let mut warrior = warrior_at_level(7);
    warrior.shoot();
}

// Backward Actions

#[test]
fn test_walk_backward() {
    let mut warrior = warrior_at_level(6);
    warrior.walk_toward(Direction::Backward);
    assert_eq!(warrior.action(), Some(Action::Walk(Direction::Backward)));
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_walk_backward_not_unlocked() {
    let mut warrior = warrior_at_level(5);
    warrior.walk_toward(Direction::Backward);
}

#[test]
fn test_attack_backward() {
    let mut warrior = warrior_at_level(6);
    warrior.attack_toward(Direction::Backward);
    assert_eq!(warrior.action(), Some(Action::Attack(Direction::Backward)));
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_attack_backward_not_unlocked() {
    let mut warrior = warrior_at_level(5);
    warrior.attack_toward(Direction::Backward);
}

#[test]
fn test_rescue_backward() {
    let mut warrior = warrior_at_level(6);
    warrior.rescue_toward(Direction::Backward);
    assert_eq!(warrior.action(), Some(Action::Rescue(Direction::Backward)));
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_rescue_backward_not_unlocked() {
    let mut warrior = warrior_at_level(5);
    warrior.rescue_toward(Direction::Backward);
}

#[test]
fn test_pivot_backward() {
    let mut warrior = warrior_facing_backward(7);
    warrior.pivot();
    assert_eq!(warrior.action(), Some(Action::Pivot(Direction::Forward)));
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_pivot_backward_not_unlocked() {
    let mut warrior = warrior_at_level(6);
    warrior.pivot();
}

#[test]
fn test_shoot_backward() {
    let mut warrior = warrior_at_level(8);
    warrior.shoot_toward(Direction::Backward);
    assert_eq!(warrior.action(), Some(Action::Shoot(Direction::Backward)));
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_shoot_backward_not_unlocked() {
    let mut warrior = warrior_at_level(7);
    warrior.shoot_toward(Direction::Backward);
}

// Non-Action Methods

#[test]
fn test_check() {
    let warrior = warrior_at_level(2);
    assert_eq!(warrior.check(), Tile::Wall);

    let ahead = [Tile::Empty, Tile::Stairs, Tile::Unit(UnitType::Sludge)];

    for tile in &ahead {
        let warrior = warrior_with_ahead(2, vec![*tile]);
        assert_eq!(warrior.check(), *tile);
    }
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_check_not_unlocked() {
    let warrior = warrior_at_level(1);
    warrior.check();
}

#[test]
fn test_look() {
    let ahead = vec![Tile::Empty, Tile::Empty, Tile::Stairs];
    let warrior = warrior_with_ahead(8, ahead.clone());
    assert_eq!(warrior.look(), &ahead);
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_look_not_unlocked() {
    let warrior = warrior_at_level(7);
    warrior.look();
}

#[test]
fn test_health() {
    let warrior = warrior_with_health(3, 10);
    assert_eq!(warrior.health(), 10);
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_health_not_unlocked() {
    let warrior = warrior_at_level(2);
    warrior.health();
}

// Backward Non-Action Methods

#[test]
fn test_check_backward() {
    let warrior = warrior_at_level(6);
    assert_eq!(warrior.check_toward(Direction::Backward), Tile::Wall);

    let behind = [Tile::Empty, Tile::Stairs, Tile::Unit(UnitType::Sludge)];

    for tile in &behind {
        let warrior = warrior_with_behind(6, vec![*tile]);
        assert_eq!(warrior.check_toward(Direction::Backward), *tile);
    }
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_check_backward_not_unlocked() {
    let warrior = warrior_at_level(5);
    warrior.check_toward(Direction::Backward);
}

#[test]
fn test_look_backward() {
    let behind = vec![Tile::Empty, Tile::Empty, Tile::Stairs];
    let warrior = warrior_with_behind(8, behind.clone());
    assert_eq!(warrior.look_toward(Direction::Backward), &behind);
}

#[test]
#[should_panic(expected = "You have not yet learned")]
fn test_look_backward_not_unlocked() {
    let warrior = warrior_at_level(7);
    warrior.look_toward(Direction::Backward);
}

// Test Helpers

fn warrior_at_level(level: usize) -> Warrior {
    Warrior::new(level, Vec::new(), Vec::new(), 0, Direction::Forward)
}

fn warrior_with_ahead(level: usize, ahead: Vec<Tile>) -> Warrior {
    Warrior::new(level, ahead, Vec::new(), 0, Direction::Forward)
}

fn warrior_with_behind(level: usize, behind: Vec<Tile>) -> Warrior {
    Warrior::new(level, Vec::new(), behind, 0, Direction::Forward)
}

fn warrior_with_health(level: usize, health: i32) -> Warrior {
    Warrior::new(level, Vec::new(), Vec::new(), health, Direction::Forward)
}

fn warrior_facing_backward(level: usize) -> Warrior {
    Warrior::new(level, Vec::new(), Vec::new(), 0, Direction::Backward)
}
