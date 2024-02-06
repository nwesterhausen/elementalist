use bevy::prelude::*;
use rand::Rng;

use crate::state::Game;

use super::resources::GenerationSeed;

pub(super) fn generate_new_seed(mut seed: ResMut<GenerationSeed>) {
    let rng = &mut rand::thread_rng();
    seed.0 = rng.gen();
}

pub(super) fn progress_to_playing(mut state: ResMut<NextState<Game>>) {
    state.set(Game::Playing);
}
