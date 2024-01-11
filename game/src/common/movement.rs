use bevy::prelude::*;

/// Simple velocity component containing a 2D vector
///
/// Defaults to Vec2::ZERO
#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec2,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { value: Vec2::ZERO }
    }
}

impl Velocity {
    /// Creates a new velocity component with the given value
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}

/// Simple acceleration component containing a 2D vector
///
/// Defaults to Vec2::ZERO
#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec2,
}

impl Default for Acceleration {
    fn default() -> Self {
        Self { value: Vec2::ZERO }
    }
}

impl Acceleration {
    /// Creates a new acceleration component with the given value
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}

/// Bundle that contains all components needed for moving things
#[derive(Bundle, Debug, Default)]
pub struct MovingThingBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

/// Plugin that makes moving things move
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_velocity, update_position));
    }
}

/// System that updates the velocity of moving things
fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in &mut query {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

/// System that updates the position of moving things
fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.value.x * time.delta_seconds();
        transform.translation.y += velocity.value.y * time.delta_seconds();
    }
}
