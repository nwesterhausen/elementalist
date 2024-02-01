use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use game_library::data_loader::ParticleEffectStore;

use crate::resources::AppState;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        // Add Hanabi plugin to the app
        app.add_plugins(HanabiPlugin);

        // Add the particle systems
        app.add_systems(OnEnter(AppState::MainMenu), example_particles);
    }
}

// Example 2d particle system from bevy_hanabi
fn example_particles(mut commands: Commands, particle_effects: Res<ParticleEffectStore>) {
    let Some(effect) = particle_effects.particles.get("2d") else {
        tracing::error!("Failed to load particle effect: 2d");
        return;
    };

    // Spawn an instance of the particle effect, and override its Z layer to
    // be above the reference white square previously spawned.
    commands
        .spawn(ParticleEffectBundle {
            // Assign the Z layer so it appears in the egui inspector and can be modified at runtime
            effect: ParticleEffect::new(effect.clone()).with_z_layer_2d(Some(10.1)),
            ..default()
        })
        .insert(Name::new("effect:2d"));
}
