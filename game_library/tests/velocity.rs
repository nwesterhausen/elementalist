use bevy::math::Vec2;
use game_library::Velocity;

#[test]
fn velocity() {
    let mut velocity = Velocity::new(Vec2::new(1.0, 1.0));

    assert_eq!(velocity.value, Vec2::new(1.0, 1.0));

    velocity.value = Vec2::new(2.0, 2.0);

    assert_eq!(velocity.value, Vec2::new(2.0, 2.0));
}
