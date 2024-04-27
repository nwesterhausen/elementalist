use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use elementalist_game_library::{
    state::{Game, Overlay},
    Acceleration,
};

/// Plugin that makes moving things move
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_velocity
                .run_if(in_state(Game::Playing).and_then(not(in_state(Overlay::Settings)))),
        );
    }
}

/// System that updates the velocity of moving things
fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in &mut query {
        velocity.linvel += acceleration.value * time.delta_seconds();
    }
}
