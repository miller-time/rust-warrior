use rust_warrior::{actions::Action, Direction, Tile, UnitType, Warrior};

// Actions

#[test]
fn test_walk() {
    let mut warrior = Warrior::default();
    warrior.walk();
    assert_eq!(warrior.action, Some(Action::Walk(Direction::Forward)));
}

#[test]
fn test_attack() {
    let mut warrior = Warrior::default();
    warrior.attack();
    assert_eq!(warrior.action, Some(Action::Attack(Direction::Forward)));
}

#[test]
fn test_rest() {
    let mut warrior = Warrior::default();
    warrior.rest();
    assert_eq!(warrior.action, Some(Action::Rest));
}

#[test]
fn test_rescue() {
    let mut warrior = Warrior::default();
    warrior.rescue();
    assert_eq!(warrior.action, Some(Action::Rescue(Direction::Forward)));
}

#[test]
fn test_pivot() {
    let mut warrior = Warrior::default();
    warrior.pivot();
    assert_eq!(warrior.action, Some(Action::Pivot(Direction::Backward)));
}

#[test]
fn test_shoot() {
    let mut warrior = Warrior::default();
    warrior.shoot();
    assert_eq!(warrior.action, Some(Action::Shoot(Direction::Forward)));
}

// Backward Actions

#[test]
fn test_walk_backward() {
    let mut warrior = Warrior::default();
    warrior.walk_toward(Direction::Backward);
    assert_eq!(warrior.action, Some(Action::Walk(Direction::Backward)));
}

#[test]
fn test_attack_backward() {
    let mut warrior = Warrior::default();
    warrior.attack_toward(Direction::Backward);
    assert_eq!(warrior.action, Some(Action::Attack(Direction::Backward)));
}

#[test]
fn test_rescue_backward() {
    let mut warrior = Warrior::default();
    warrior.rescue_toward(Direction::Backward);
    assert_eq!(warrior.action, Some(Action::Rescue(Direction::Backward)));
}

#[test]
fn test_pivot_backward() {
    let mut warrior = warrior_facing_backward();
    warrior.pivot();
    assert_eq!(warrior.action, Some(Action::Pivot(Direction::Forward)));
}

#[test]
fn test_shoot_backward() {
    let mut warrior = Warrior::default();
    warrior.shoot_toward(Direction::Backward);
    assert_eq!(warrior.action, Some(Action::Shoot(Direction::Backward)));
}

// Non-Action Methods

#[test]
fn test_check() {
    let warrior = Warrior::default();
    assert_eq!(warrior.check(), Tile::Wall);

    let ahead = [Tile::Empty, Tile::Stairs, Tile::Unit(UnitType::Sludge)];

    for tile in &ahead {
        let warrior = warrior_with_ahead(vec![*tile]);
        assert_eq!(warrior.check(), *tile);
    }
}

#[test]
fn test_look() {
    let ahead = vec![Tile::Empty, Tile::Empty, Tile::Stairs];
    let warrior = warrior_with_ahead(ahead.clone());
    assert_eq!(warrior.look(), &ahead);
}

#[test]
fn test_health() {
    let warrior = warrior_with_health(10);
    assert_eq!(warrior.health(), 10);
}

// Backward Non-Action Methods

#[test]
fn test_check_backward() {
    let warrior = Warrior::default();
    assert_eq!(warrior.check_toward(Direction::Backward), Tile::Wall);

    let behind = [Tile::Empty, Tile::Stairs, Tile::Unit(UnitType::Sludge)];

    for tile in &behind {
        let warrior = warrior_with_behind(vec![*tile]);
        assert_eq!(warrior.check_toward(Direction::Backward), *tile);
    }
}

#[test]
fn test_look_backward() {
    let behind = vec![Tile::Empty, Tile::Empty, Tile::Stairs];
    let warrior = warrior_with_behind(behind.clone());
    assert_eq!(warrior.look_toward(Direction::Backward), &behind);
}

// Test Helpers

fn warrior_with_ahead(ahead: Vec<Tile>) -> Warrior {
    Warrior::new(ahead, Vec::new(), 0, Direction::Forward)
}

fn warrior_with_behind(behind: Vec<Tile>) -> Warrior {
    Warrior::new(Vec::new(), behind, 0, Direction::Forward)
}

fn warrior_with_health(health: i32) -> Warrior {
    Warrior::new(Vec::new(), Vec::new(), health, Direction::Forward)
}

fn warrior_facing_backward() -> Warrior {
    Warrior::new(Vec::new(), Vec::new(), 0, Direction::Backward)
}
