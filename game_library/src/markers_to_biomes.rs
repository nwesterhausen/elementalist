//! Trait to convert a map of `Marker` to appropriate biomes from a `Realm`

use crate::{
    enums::biome::{Biome, Marker},
    BiomeData, Realm,
};

/// Trait to convert a map of `Marker` to appropriate biomes from a `Realm`
pub trait MarkersToBiomes {
    /// Convert a map of `Marker` to appropriate biomes from a `Realm`
    ///
    /// This takes into account the `Realm`'s `BiomeData` and the `Marker`'s `Biome`
    /// which is elevation based. First we look at what biomes are available in the
    /// `Realm` and then come up with how to match them to the 20 possible elevations
    /// from the `Marker` map.
    fn markers_to_biomes(&self, marker_map: &[Vec<Marker>]) -> Vec<Vec<Biome>>;
}

impl MarkersToBiomes for Realm {
    fn markers_to_biomes(&self, marker_map: &[Vec<Marker>]) -> Vec<Vec<Biome>> {
        // Determine what biomes are available in the realm, and order them by altitude
        // and humidity.
        let mut biomes = self.biomes.clone();
        biomes.sort_by(|a, b| {
            let a = a.altitude as u8 * 10 + a.humidity as u8;
            let b = b.altitude as u8 * 10 + b.humidity as u8;
            a.cmp(&b)
        });
        // todo: capture the other biome details when we transform to a map..
        // align the biomes to the markers
        let aligned = align_biomes_to_markers(biomes);

        // Convert the markers to biomes
        let mut result = Vec::<Vec<Biome>>::new();
        for row in marker_map {
            let mut row_biomes = Vec::<Biome>::new();
            for marker in row {
                let idx = marker.as_elevation_idx();
                if idx >= aligned.len() {
                    tracing::error!("markers_to_biomes: idx out of bounds {idx}");
                    row_biomes.push(aligned[0]);
                } else {
                    row_biomes.push(aligned[idx]);
                }
            }
            result.push(row_biomes);
        }

        result
    }
}

/// Align a list of biomes to the `Marker` enum. This just takes the first 20
/// biomes and aligns them to the 20 possible elevations from the `Marker` map.
/// If there are less than 20 biomes, they are evenly distributed across the
/// elevations.
///
/// # Arguments
///
/// * `biomes` - The list of biomes to align to the `Marker` enum.
///
/// # Returns
///
/// A list of biomes aligned to the `Marker` enum.
#[must_use]
pub fn align_biomes_to_markers(biomes: Vec<BiomeData>) -> [Biome; 20] {
    let mut aligned = Vec::<Biome>::new();

    // If there are less than 20 biomes, evenly distribute them across the elevations
    if biomes.len() < 20 {
        for (i, _) in (0..20).enumerate() {
            let idx = i % biomes.len();
            // Sanity check
            if idx >= biomes.len() {
                tracing::warn!("align_biomes_to_markers: idx unexpectedly out of bounds: {idx}");
                break;
            }
            aligned.push(biomes[idx].biome);
        }
    } else {
        for biome in biomes {
            aligned.push(biome.biome);
        }
    }

    // Convert the vector to an array
    let mut array = [Biome::Barren; 20];
    for (i, biome) in aligned.iter().enumerate() {
        if i >= 20 {
            break;
        }
        array[i] = *biome;
    }
    array
}
