use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::state::Game;

/// Pixels per meter
pub const PIXELS_PER_METER: f32 = 16.0;
/// Gravity (in pixels per second squared); No gravity.
pub const GRAVITY: Vec2 = Vec2::ZERO;

/// Elementalist physics plugin
#[allow(clippy::module_name_repetitions)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Add the Rapier physics plugin
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .insert_resource(RapierConfiguration {
            gravity: GRAVITY,
            ..default()
        });

        app.add_systems(Update, display_events.run_if(in_state(Game::Playing)));

        #[cfg(debug_assertions)]
        {
            // debug renderer
            app.add_plugins(RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            });
            // debug renderer toggle
            app.add_systems(Update, toggle_debug_rendering);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn toggle_debug_rendering(
    mut debug_render_context: ResMut<DebugRenderContext>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        debug_render_context.enabled = !debug_render_context.enabled;
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        info!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        info!("Received contact force event: {:?}", contact_force_event);
    }
}
