use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::{enums::biome::Marker, state::Game};

use super::resources::{GeneratedMaps, GenerationSeed};

/// Boundary for the object id noise generation. This is set as both the upper and lower bounds,
/// with the lower bound being negative.
pub const OBJECT_BOUNDARY: f64 = 2.5;
/// Weight pool for object generation. We turn the noise into a usize that is within the range
/// of zero to this value.
///
/// It should be taken into account that some of the pool space should be used for empty space,
/// otherwise the map will be too cluttered.
pub const OBJECT_POOL: usize = 200;

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
/// When that sequence is complete, the game state should change from `Game::Generating` to
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
    let mut small_rng = SmallRng::seed_from_u64(seed.as_u64());

    let width = maps.dimensions().0;
    let height = maps.dimensions().1;

    maps.reset();

    for x in 0..width {
        for y in 0..height {
            // We allow precision loss here because it's a very edge case where it would matter.
            #[allow(clippy::cast_precision_loss)]
            let pos = [x as f64 / 100.0, y as f64 / 100.0, 0.0];
            let value = perlin.get(pos);
            let biome = Marker::from_noise(value);
            maps.biome_map[x].push(biome);

            // Map the small_rng noise for position to the object map.
            let value = small_rng.gen_range(-OBJECT_BOUNDARY..OBJECT_BOUNDARY);
            let object = noise_to_object(value);
            maps.object_map[x].push(object);
        }
    }

    tracing::info!(
        "Generated {}x{} map",
        maps.biome_map.len(),
        maps.biome_map[0].len()
    );
}

/// Take noise and return a usize between 0 and `OBJECT_POOL`.
///
/// # Arguments
///
/// * `noise` - The noise value to convert to an object marker. This is expected to be
/// between -`OBJECT_BOUNDARY` and `OBJECT_BOUNDARY`. Anything outside of this range will
/// be clamped to the nearest boundary.
///
/// # Returns
///
/// A usize between 0 and `OBJECT_POOL`.
#[must_use]
fn noise_to_object(noise: f64) -> usize {
    let noise = noise.clamp(-OBJECT_BOUNDARY, OBJECT_BOUNDARY);
    let noise = (noise + OBJECT_BOUNDARY) / (OBJECT_BOUNDARY * 2.0);
    #[allow(clippy::cast_sign_loss)]
    let object = (noise * OBJECT_POOL as f64) as usize;

    object
}
