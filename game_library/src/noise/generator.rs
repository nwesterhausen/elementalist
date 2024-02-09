use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Simplex};
use rand::Rng;

use crate::{enums::GenericBiome, state::Game};

use super::resources::{GeneratedMaps, GenerationSeed};

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

/// Use the perlin noise generator to generate a biome map. And then use simplex noise to determine
/// how to place trees and other objects based on the biome map. Both should be seedable.
///
/// This system should be called when the game state is `Game::Generating`.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn generate_map(seed: Res<GenerationSeed>, mut maps: ResMut<GeneratedMaps>) {
    tracing::info!("Generating map with seed: {}", seed.0);

    let perlin = Perlin::new(seed.0);
    let simplex = Simplex::new(seed.0);

    let width = maps.dimensions().0;
    let height = maps.dimensions().1;

    maps.reset();

    for x in 0..width {
        for y in 0..height {
            // We allow precision loss here because it's a very edge case where it would matter.
            #[allow(clippy::cast_precision_loss)]
            let pos = [x as f64 / 100.0, y as f64 / 100.0, 0.0];
            let value = perlin.get(pos);
            let biome = match value {
                v if v < -0.5 => GenericBiome::Biome1,
                v if v < -0.4 => GenericBiome::Biome2,
                v if v < -0.3 => GenericBiome::Biome3,
                v if v < -0.2 => GenericBiome::Biome4,
                v if v < -0.1 => GenericBiome::Biome5,
                v if v < 0.0 => GenericBiome::Biome6,
                v if v < 0.1 => GenericBiome::Biome7,
                v if v < 0.2 => GenericBiome::Biome8,
                v if v < 0.3 => GenericBiome::Biome9,
                _ => GenericBiome::Biome10,
            };
            maps.biome_map[x].push(biome);

            // Map the simplex noise for position to the object map.
            let value = simplex.get(pos);
            let object = match value {
                v if v < -0.5 => 0,
                v if v < -0.4 => 1,
                v if v < -0.3 => 2,
                v if v < -0.2 => 3,
                v if v < -0.1 => 4,
                v if v < 0.0 => 5,
                v if v < 0.1 => 6,
                v if v < 0.2 => 7,
                v if v < 0.3 => 8,
                _ => 9,
            };
            maps.object_map[x].push(object);
        }
    }

    tracing::info!(
        "Generated {}x{} map",
        maps.biome_map.len(),
        maps.biome_map[0].len()
    );
}
