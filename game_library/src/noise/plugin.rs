use bevy::prelude::*;

use crate::state::Game;

use super::{
    beta_info::draw_seed_on_screen,
    generator::{generate_biome_map, generate_new_seed, progress_to_playing},
    resources::{GeneratedMaps, GenerationSeed},
};

/// A plugin that provides the resources and systems for the noise generation.
#[allow(clippy::module_name_repetitions)]
pub struct NoisePlugin;

impl Plugin for NoisePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GenerationSeed>()
            .init_resource::<GeneratedMaps>()
            .add_systems(Update, draw_seed_on_screen.run_if(in_state(Game::Playing)))
            .add_systems(
                OnEnter(Game::Generating),
                (generate_new_seed, generate_biome_map, progress_to_playing).chain(),
            );
    }
}
