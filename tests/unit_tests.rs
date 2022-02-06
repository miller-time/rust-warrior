use rust_warrior::UnitType;

#[test]
fn test_unit_type_draw() {
    let u = UnitType::Archer;
    assert_eq!(u.draw(), "a");

    let u = UnitType::Captive;
    assert_eq!(u.draw(), "C");

    let u = UnitType::Sludge;
    assert_eq!(u.draw(), "s");

    let u = UnitType::ThickSludge;
    assert_eq!(u.draw(), "S");

    let u = UnitType::Warrior;
    assert_eq!(u.draw(), "@");

    let u = UnitType::Wizard;
    assert_eq!(u.draw(), "w");
}
