//! Defines the `BiomeData` resource.
use bevy::prelude::*;
use bevy::reflect::Reflect;

use crate::enums::biome::{Altitude, Biome, Humidity, Latitude};

/// The biome system is a list of 1 - 10 "biomes" that are then used to determine the actual
/// biome of the world. This is then used to determine the type of terrain and the type of
/// objects that are placed in the world. This is then used to determine the actual biome
/// of the world.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Resource,
    Reflect,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct BiomeData {
    /// The actual biome of the world.
    pub biome: Biome,
    /// The altitude (and temperature) of the biome.
    pub altitude: Altitude,
    /// The humidity of the biome.
    pub humidity: Humidity,
    /// The latitudinal band of the biome.
    pub latitude: Latitude,
}
