use bevy::prelude::*;
use game_library::{
    state::{Game, Overlay},
    Acceleration, Velocity,
};

/// Plugin that makes moving things move
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_position)
                .run_if(in_state(Game::Playing).and_then(not(in_state(Overlay::Settings)))),
        );
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
