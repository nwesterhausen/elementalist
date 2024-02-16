use bevy::prelude::*;
use bevy_rapier2d::control::KinematicCharacterController;
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

/// System that updates the position of moving things (via physics)
fn update_position(
    mut query: Query<(&mut KinematicCharacterController, &Velocity)>,
    time: Res<Time>,
) {
    for (mut controller, velocity) in &mut query {
        controller.translation = Some(Vec2::new(
            velocity.value.x * time.delta_seconds(),
            velocity.value.y * time.delta_seconds(),
        ));
    }
}
