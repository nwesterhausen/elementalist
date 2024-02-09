/// A base biome enum. This is then further used with the type of primal realm
/// to determine the actual biome. This biome is set by the noise generator, and
/// it describes the "height" of the terrain, and that should then be interpreted
/// later (see the impls of this enum) to determine the actual biome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenericBiome {
    /// The lowest possible biome.
    Biome1,
    /// The second lowest possible biome.
    Biome2,
    /// The third lowest possible biome.
    Biome3,
    /// The fourth lowest possible biome.
    Biome4,
    /// The lower-middle possible biome.
    Biome5,
    /// The higher-middle possible biome.
    Biome6,
    /// The fourth highest possible biome.
    Biome7,
    /// The third highest possible biome.
    Biome8,
    /// The second highest possible biome.
    Biome9,
    /// The highest possible biome.
    Biome10,
}

/// The actual biomes that are used in the game. These are then used to draw the terrain
/// and to determine the type of objects that are placed in the world. Since this the number
/// of actual biomes is much greater than the number of generic biomes, to determine the actual
/// biome the Realm type needs to be used with the `biome_for_realm` method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Biome {
    /// A mostly empty grassland.
    Grassland,
    /// Deciduous forest (least dense).
    DeciduousForest,
    /// Deciduous forest (more dense).
    DeciduousForestDense,
    /// Coniferous forest (least dense).
    ConiferousForest,
    /// Coniferous forest (more dense).
    ConiferousForestDense,
    /// A mostly empty desert.
    Desert,
    /// A mostly empty tundra.
    Tundra,
    /// A mostly empty taiga.
    Taiga,
    /// A mostly empty savanna.
    Savanna,
    /// Topical rainforest (least dense).
    Rainforest,
    /// Topical rainforest (more dense).
    RainforestDense,
    // todo: add more biomes
}
