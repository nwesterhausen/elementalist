use game_library::Stat;

#[test]
fn new() {
    let stat = Stat::new(1.0);
    let almost_eq = (stat.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn add_base_value() {
    let mut stat = Stat::new(1.0);

    let almost_eq = (stat.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.add_base_value(2.0);
    let almost_eq = (stat.value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn add_base_value_below_zero() {
    let mut stat = Stat::new(1.0);

    let almost_eq = (stat.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.add_base_value(-2.0);
    let almost_eq = (stat.value() - -0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - -1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn subtract_base_value() {
    let mut stat = Stat::new(1.0);

    let almost_eq = (stat.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.subtract_base_value(0.25);
    let almost_eq = (stat.value() - 0.75).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 0.75).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn multiply_base_value() {
    let mut stat = Stat::new(1.0);

    let almost_eq = (stat.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.multiply_base_value(2.0);
    let almost_eq = (stat.value() - 2.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 2.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn multiply_base_value_below_zero() {
    let mut stat = Stat::new(1.0);

    let almost_eq = (stat.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.multiply_base_value(-2.0);
    let almost_eq = (stat.value() - 0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - -2.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn divide_base_value() {
    let mut stat = Stat::new(1.0);

    let almost_eq = (stat.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.divide_base_value(2.0);
    let almost_eq = (stat.value() - 0.5).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 0.5).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn divide_base_value_below_zero() {
    let mut stat = Stat::new(1.0);

    let almost_eq = (stat.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.divide_base_value(-2.0);
    let almost_eq = (stat.value() - -0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - -0.5).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn divide_base_by_zero() {
    let mut stat = Stat::new(1.0);

    let almost_eq = (stat.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.divide_base_value(0.0);
    let almost_eq = (stat.value() - 0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn add_bonus() {
    let mut stat = Stat::new(1.0);

    let almost_eq = (stat.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.add_bonus(2.0);
    let almost_eq = (stat.value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn add_bonus_below_zero() {
    let mut stat = Stat::new(1.0);

    let almost_eq = (stat.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.add_bonus(-2.0);
    let almost_eq = (stat.value() - -0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - -0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn subtract_bonus() {
    let mut stat = Stat::new(1.0);
    stat.add_base_value(2.0);

    let almost_eq = (stat.value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);

    stat.subtract_bonus(0.25);

    let almost_eq = (stat.value() - 2.25).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 0.75).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn multiply_bonus() {
    let mut stat = Stat::new(1.0);
    stat.add_base_value(2.0);

    let almost_eq = (stat.value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);

    stat.multiply_bonus(2.0);

    let almost_eq = (stat.value() - 6.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 2.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn multiply_bonus_below_zero() {
    let mut stat = Stat::new(1.0);
    stat.add_base_value(2.0);

    let almost_eq = (stat.value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);

    stat.multiply_bonus(-2.0);

    let almost_eq = (stat.value() - 0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - -0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn divide_bonus() {
    let mut stat = Stat::new(1.0);
    stat.add_base_value(2.0);
    let almost_eq = (stat.value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.divide_bonus(2.0);
    let almost_eq = (stat.value() - 1.5).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 0.5).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn divide_bonus_below_zero() {
    let mut stat = Stat::new(1.0);
    stat.add_base_value(2.0);
    let almost_eq = (stat.value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.divide_bonus(-2.0);
    let almost_eq = (stat.value() - 0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - -0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn divide_bonus_by_zero() {
    let mut stat = Stat::new(1.0);
    stat.add_base_value(2.0);
    let almost_eq = (stat.value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    stat.divide_bonus(0.0);
    let almost_eq = (stat.value() - 0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 3.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn from_f32() {
    let stat = Stat::from(1.0);
    let almost_eq = (stat.value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.base_value() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (stat.bonus() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
}
