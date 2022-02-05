use rust_warrior::{floor::Floor, unit::Unit, Tile, UnitType};

#[test]
fn test_tile_draw() {
    let t = Tile::Empty;
    assert_eq!(t.draw(), " ");

    let t = Tile::Stairs;
    assert_eq!(t.draw(), ">");

    let t = Tile::Unit(UnitType::Warrior);
    assert_eq!(t.draw(), "@");
}

#[test]
fn test_floor_tile() {
    let f = Floor {
        width: 3,
        height: 1,
        stairs: (2, 0),
        units: vec![Unit::warrior((0, 0))],
    };

    let t = f.tile((0, 0));
    assert_eq!(t, Tile::Unit(UnitType::Warrior));

    let t = f.tile((1, 0));
    assert_eq!(t, Tile::Empty);

    let t = f.tile((2, 0));
    assert_eq!(t, Tile::Stairs);
}

#[test]
fn test_floor_load() {
    let f = Floor::load(1);

    let t = f.tile((0, 0));
    assert_eq!(t, Tile::Unit(UnitType::Warrior));

    for x in 1..7 {
        let t = f.tile((x, 0));
        assert_eq!(t, Tile::Empty);
    }

    let t = f.tile((7, 0));
    assert_eq!(t, Tile::Stairs);
}
