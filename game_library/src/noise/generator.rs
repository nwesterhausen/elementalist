use bevy::prelude::*;
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
