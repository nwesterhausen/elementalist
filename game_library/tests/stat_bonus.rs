use game_library::StatBonus;

#[test]
fn new() {
    let stat_bonus = StatBonus::new(1.0);
    let almost_eq = (stat_bonus.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn set_value() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.set_value(2.0);
    assert_eq!(stat_bonus.value(), 2.0);
}

#[test]
fn set_value_below_zero() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.set_value(-2.0);
    assert_eq!(stat_bonus.value(), 0.0);
}

#[test]
fn add_value() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.add_value(2.0);
    assert_eq!(stat_bonus.value(), 3.0);
}

#[test]
fn add_value_below_zero() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.add_value(-2.0);
    assert_eq!(stat_bonus.value(), 0.0);
}

#[test]
fn subtract_value() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.subtract_value(0.25);
    assert_eq!(stat_bonus.value(), 0.75);
}

#[test]
fn subtract_value_below_zero() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.subtract_value(2.0);
    assert_eq!(stat_bonus.value(), 0.0);
}

#[test]
fn multiply_value() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.multiply_value(2.0);
    assert_eq!(stat_bonus.value(), 2.0);
}

#[test]
fn multiply_value_below_zero() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.multiply_value(-2.0);
    assert_eq!(stat_bonus.value(), 0.0);
}

#[test]
fn divide_value() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.divide_value(2.0);
    assert_eq!(stat_bonus.value(), 0.5);
}

#[test]
fn divide_value_below_zero() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.divide_value(-2.0);
    assert_eq!(stat_bonus.value(), 0.0);
}

#[test]
fn divide_by_zero() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.divide_value(0.0);
    assert_eq!(stat_bonus.value(), 0.0);
}

#[test]
fn add_percent() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.add_percent(50.);
    assert_eq!(stat_bonus.value(), 1.5);
}

#[test]
fn subtract_percent() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.subtract_percent(50.);
    assert_eq!(stat_bonus.value(), 0.5);
}

#[test]
fn multiply_percent() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.multiply_percent(12.);
    assert_eq!(stat_bonus.value(), 0.12);
}

#[test]
fn divide_percent() {
    let mut stat_bonus = StatBonus::new(1.0);
    assert_eq!(stat_bonus.value(), 1.0);
    stat_bonus.divide_percent(30.);
    assert_eq!(stat_bonus.value(), 3.3333333);
}
