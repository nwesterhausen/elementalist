use elementalist_game_library::Attribute;

#[test]
fn new() {
    let attribute = Attribute::new(100_u32);

    assert_eq!(attribute.current, 100_u32);
    assert!(u32::from(attribute) == 100_u32);
    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn new_with_current() {
    let attribute = Attribute::new_with_current(20_u32, 100_u32);

    assert_eq!(attribute.current, 20_u32);
    assert!(u32::from(attribute) == 20_u32);
    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.2).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 20);
}

#[test]
fn new_with_current_greater_than_max() {
    let attribute = Attribute::new_with_current(200_u32, 100_u32);

    assert_eq!(attribute.current, 100_u32);
    assert!(u32::from(attribute) == 100_u32);
    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn add_to_current() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.add_to_current(10_u32);

    assert_eq!(attribute.current, 30_u32);
    assert!(u32::from(attribute) == 30_u32);
    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.3).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 30);
}

#[test]
fn add_to_current_greater_than_max() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.add_to_current(100_u32);

    assert_eq!(attribute.current, 100_u32);
    assert!(u32::from(attribute) == 100_u32);
    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn add_to_current_negative() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.add_to_current(-10_i32);

    assert_eq!(attribute.current, 10_u32);
    assert!(u32::from(attribute) == 10_u32);
    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.1).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 10);
}

#[test]
fn add_to_current_negative_below_zero() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.add_to_current(-100_i32);

    assert_eq!(attribute.current, 0_u32);
    assert!(u32::from(attribute) == 0_u32);
    assert!(attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 0);
}

#[test]
fn add_to_max() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.add_to_max(10_u32);

    assert_eq!(attribute.current, 20_u32);
    assert!(u32::from(attribute) == 20_u32);

    assert_eq!(attribute.max, 110_u32);
    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.18).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 18);
}

#[test]
fn add_to_max_negative() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.add_to_max(-10_i32);

    assert_eq!(attribute.current, 20_u32);
    assert!(u32::from(attribute) == 20_u32);

    assert_eq!(attribute.max, 90_u32);
    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.22).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 22);
}

#[test]
fn add_to_max_negative_below_current() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.add_to_max(-90_i32);

    assert_eq!(attribute.current, 10_u32);
    assert!(u32::from(attribute) == 10_u32);

    assert_eq!(attribute.max, 10_u32);
    assert_eq!(attribute.current, attribute.max);
    assert!(!attribute.is_empty());
    assert!(attribute.is_full());

    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn add_to_max_negative_below_zero() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.add_to_max(-100_i32);

    assert_eq!(attribute.current, 0_u32);
    assert!(u32::from(attribute) == 0_u32);

    assert_eq!(attribute.max, 0_u32);
    assert!(attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn scale_current() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.scale_current(2.);

    assert_eq!(attribute.current, 40_u32);
    assert!(u32::from(attribute) == 40_u32);
    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.4).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 40);
}

#[test]
fn scale_current_greater_than_max() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.scale_current(10.);

    assert_eq!(attribute.current, 100_u32);
    assert!(u32::from(attribute) == 100_u32);
    assert!(!attribute.is_empty());
    assert!(attribute.is_full());

    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn scale_max() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.scale_max(2.);

    assert_eq!(attribute.current, 20_u32);
    assert!(u32::from(attribute) == 20_u32);

    assert_eq!(attribute.max, 200_u32);
    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.1).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 10);
}

#[test]
fn scale_current_by_percentage() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.scale_current_by_percentage(50);

    assert_eq!(attribute.current, 10_u32);
    assert!(u32::from(attribute) == 10_u32);
    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.1).abs() < f32::EPSILON;
    assert!(almost_eq);

    assert_eq!(attribute.percentage_remaining(), 10);
}

#[test]
fn scale_current_by_percentage_greater_than_max() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.scale_current_by_percentage(800);

    assert_eq!(attribute.current, 100_u32);
    assert!(u32::from(attribute) == 100_u32);
    assert!(!attribute.is_empty());
    assert!(attribute.is_full());

    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn scale_max_by_percentage() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.scale_max_by_percentage(50);

    assert_eq!(attribute.current, 20_u32);
    assert!(u32::from(attribute) == 20_u32);

    assert_eq!(attribute.max, 50_u32);
    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.4).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 40);
}

#[test]
fn add_to_current_by_scale_max_percentage() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.add_to_current_by_scale_max_percentage(50);

    assert_eq!(attribute.current, 70_u32);
    assert!(u32::from(attribute) == 70_u32);
    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.7).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 70);
}

#[test]
fn add_to_current_by_scale_max_percentage_greater_than_max() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.add_to_current_by_scale_max_percentage(800);

    assert_eq!(attribute.current, 100_u32);
    assert!(u32::from(attribute) == 100_u32);
    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn add_to_current_by_scale_max() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.add_to_current_by_scale_max(0.5);

    assert_eq!(attribute.current, 70_u32);
    assert!(u32::from(attribute) == 70_u32);
    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.7).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 70);
}

#[test]
fn add_to_current_by_scale_max_greater_than_max() {
    let mut attribute = Attribute::new_with_current(20_u32, 100_u32);

    attribute.add_to_current_by_scale_max(8.);

    assert_eq!(attribute.current, 100_u32);
    assert!(u32::from(attribute) == 100_u32);

    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn from_i64() {
    let attribute = Attribute::from(20_i64);

    assert_eq!(attribute.current, 20_u32);
    assert!(u32::from(attribute) == 20_u32);

    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn from_i64_negative() {
    let attribute = Attribute::from(-20_i64);

    assert_eq!(attribute.current, 0_u32);
    assert!(u32::from(attribute) == 0_u32);

    assert!(attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn from_i32() {
    let attribute = Attribute::from(20_i32);

    assert_eq!(attribute.current, 20_u32);
    assert!(u32::from(attribute) == 20_u32);

    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn from_i32_negative() {
    let attribute = Attribute::from(-20_i32);

    assert_eq!(attribute.current, 0_u32);
    assert!(u32::from(attribute) == 0_u32);

    assert!(attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn from_i16() {
    let attribute = Attribute::from(20_i16);

    assert_eq!(attribute.current, 20_u32);
    assert!(u32::from(attribute) == 20_u32);

    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn from_i16_negative() {
    let attribute = Attribute::from(-20_i16);

    assert_eq!(attribute.current, 0_u32);
    assert!(u32::from(attribute) == 0_u32);

    assert!(attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn from_i8() {
    let attribute = Attribute::from(20_i8);

    assert_eq!(attribute.current, 20_u32);
    assert!(u32::from(attribute) == 20_u32);

    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn from_i8_negative() {
    let attribute = Attribute::from(-20_i8);

    assert_eq!(attribute.current, 0_u32);
    assert!(u32::from(attribute) == 0_u32);

    assert!(attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn from_f32() {
    let attribute = Attribute::from(2.0_f32);

    assert_eq!(attribute.current, 2_u32);
    assert!(u32::from(attribute) == 2_u32);

    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn from_f32_negative() {
    let attribute = Attribute::from(-2.0_f32);

    assert_eq!(attribute.current, 0_u32);
    assert!(u32::from(attribute) == 0_u32);

    assert!(attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn from_f64() {
    let attribute = Attribute::from(2.0_f64);

    assert_eq!(attribute.current, 2_u32);
    assert!(u32::from(attribute) == 2_u32);

    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn from_f64_negative() {
    let attribute = Attribute::from(-2.0_f64);

    assert_eq!(attribute.current, 0_u32);
    assert!(u32::from(attribute) == 0_u32);

    assert!(attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn add() {
    let mut attribute = Attribute::new_with_current(10_u32, 50_u32);

    attribute += 10_u32;

    assert_eq!(attribute.current, 20_u32);
    assert!(u32::from(attribute) == 20_u32);

    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.4).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 40);
}

#[test]
fn add_when_full() {
    let mut attribute = Attribute::new(20_u32);

    attribute += 10_u32;

    assert_eq!(attribute.current, 20_u32);
    assert!(u32::from(attribute) == 20_u32);

    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn sub() {
    let mut attribute = Attribute::new_with_current(10_u32, 50_u32);

    attribute -= 10_u32;

    assert_eq!(attribute.current, 0_u32);
    assert!(u32::from(attribute) == 0_u32);

    assert!(attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 0);
}

#[test]
fn sub_when_empty() {
    let mut attribute = Attribute::new_with_current(0_u32, 20_u32);

    attribute -= 10_u32;

    assert_eq!(attribute.current, 0_u32);
    assert!(u32::from(attribute) == 0_u32);

    assert!(attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 0);
}

#[test]
fn mul() {
    let mut attribute = Attribute::new_with_current(10_u32, 50_u32);

    attribute *= 2_u32;

    assert_eq!(attribute.current, 20_u32);
    assert!(u32::from(attribute) == 20_u32);

    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.4).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 40);
}

#[test]
fn mul_past_max() {
    let mut attribute = Attribute::new_with_current(10_u32, 50_u32);

    attribute *= 10_u32;

    assert_eq!(attribute.current, 50_u32);
    assert!(u32::from(attribute) == 50_u32);

    assert!(!attribute.is_empty());
    assert!(attribute.is_full());
    let almost_eq = (attribute.remaining() - 1.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 100);
}

#[test]
fn div() {
    let mut attribute = Attribute::new_with_current(10_u32, 50_u32);

    attribute /= 2_u32;

    assert_eq!(attribute.current, 5_u32);
    assert!(u32::from(attribute) == 5_u32);

    assert!(!attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.1).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 10);
}

#[test]
fn div_by_zero() {
    let mut attribute = Attribute::new_with_current(10_u32, 50_u32);

    attribute /= 0_u32;

    assert_eq!(attribute.current, 0_u32);
    assert!(u32::from(attribute) == 0_u32);

    assert!(attribute.is_empty());
    assert!(!attribute.is_full());
    let almost_eq = (attribute.remaining() - 0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert_eq!(attribute.percentage_remaining(), 0);
}
