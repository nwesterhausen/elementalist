use bevy::math::Vec2;
use elementalist_game_library::Acceleration;

#[test]
fn acceleration() {
    let mut acceleration = Acceleration::new(Vec2::new(1.0, 1.0));

    assert_eq!(acceleration.value, Vec2::new(1.0, 1.0));

    acceleration.value = Vec2::new(2.0, 2.0);

    assert_eq!(acceleration.value, Vec2::new(2.0, 2.0));
}
