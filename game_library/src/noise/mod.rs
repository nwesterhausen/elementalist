//! Module for procedural noise generation.
//!
//! This should take care of generating noise, tracking the seed in a resource,
//! and set up the noise resource that can be used to populate the primal realm
//! with terrain.
//!
//! The noise should be created when a new realm is created, and the seed should
//! be updated when the realm is updated. I think the best way to handle placing
//! terrain is to grab the noise resource and use a `get_tile_at` method to
//! determine what tile should be placed at a given position, and by looping over
//! the entire realm and placing tiles as needed.
//!
//! The noise should be generated using the `noise` crate, and the `Perlin` noise
//! function. The noise should be generated in chunks, and the chunks should be
//! stored in a `HashMap` with the chunk position as the key. This way, the noise
//! can be generated once and then stored for later use.
//!
//! We need integers from the noise, because we need to determine what tile to place
//! at a given position. We can use the `get_tile_at` method to determine what tile
//! to place at a given position, and then use the `Tile` to place the tile. (roughly)
mod beta_info;
mod generator;
mod plugin;
mod resources;

pub use generator::OBJECT_POOL;
#[allow(clippy::module_name_repetitions)]
pub use plugin::NoisePlugin;
pub use resources::GeneratedMaps;
