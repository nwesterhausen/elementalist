use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;

use crate::state::Game;

use super::resources::GenerationSeed;

/// Generate a new seed for the world generation.
///
/// This should be first in the sequence of events that happen when generating a new realm.
///
/// This system sets the `GenerationSeed` resource to a new random value.
pub(super) fn generate_new_seed(mut seed: ResMut<GenerationSeed>) {
    let rng = &mut rand::thread_rng();
    seed.0 = rng.gen();
    tracing::info!("New seed generated: {}", seed.0);
}

/// Progress the game state to the `Generating` state.
///
/// This is the last step in the sequence of events that happen when generating a new realm.
/// When that sequence is complete, the game state should change from `Game::Generationg` to
/// `Game::Playing`.
pub(super) fn progress_to_playing(mut state: ResMut<NextState<Game>>) {
    state.set(Game::Playing);
    tracing::info!("Progressing to playing state");
}

/// Using the seed and noise, generate the world.
///
/// This system should be run when the game state is `Game::Generating`.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn generate_world(seed: Res<GenerationSeed>, mut commands: Commands) {
    tracing::info!("Generating world with seed: {}", seed.0);

    let perlin = Perlin::new(seed.0);

    let origin = perlin.get([0.0, 0.0]);

    tracing::info!("Origin: {}", origin);
}
