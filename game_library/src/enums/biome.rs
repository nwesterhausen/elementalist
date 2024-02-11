//! Biome data and enums.
use bevy::prelude::*;
use bevy::reflect::Reflect;

/// A base biome enum. This is then further used with the type of primal realm
/// to determine the actual biome. This biome is set by the noise generator, and
/// it describes the "height" of the terrain, and that should then be interpreted
/// later (see the impls of this enum) to determine the actual biome.
///
/// Supports maps with up to 10 different biomes.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub enum Marker {
    /// An empty marker.
    #[default]
    Empty,
    /// The lowest elevation
    Elevation0,
    /// The second lowest possible biome.
    Elevation1,
    /// The third lowest possible biome.
    Elevation2,
    /// The fourth lowest possible biome.
    Elevation3,
    /// The fifth lowest possible biome.
    Elevation4,
    /// The sixth lowest possible biome.
    Elevation5,
    /// The seventh lowest possible biome.
    Elevation6,
    /// The eighth lowest possible biome.
    Elevation7,
    /// The ninth lowest possible biome.
    Elevation8,
    /// The tenth lowest possible biome.
    Elevation9,
}

/// The actual biomes that are used in the game. These are then used to draw the terrain
/// and to determine the type of objects that are placed in the world. Since this the number
/// of actual biomes is much greater than the number of generic biomes, to determine the actual
/// biome the Realm type needs to be used with the `biome_for_realm` method.
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
pub enum Biome {
    /// Rainforests are forests characterized by a closed and continuous tree canopy, moisture-dependent
    /// vegetation, the presence of epiphytes and lianas and the absence of wildfire.
    Rainforest,
    /// Seasonal tropical forest, also known as moist deciduous, semi-evergreen seasonal, tropical mixed or
    /// monsoon forest, typically contains a range of tree species: many of which drop some or all of
    /// their leaves during the dry season.
    SeasonalDeciduousRainforest,
    /// Seasonal tropical forest, also known as moist deciduous, semi-evergreen seasonal, tropical mixed or
    /// monsoon forest, typically contains a range of tree species: only some of which drop some or all of
    /// their leaves during the dry season.
    SeasonalSemidecidousRainforest,
    /// A rainforest with extra large trees.
    GiantRainforest,
    /// Deciduous or broad-leaf forests are a variety of forest 'dominated' by deciduous trees that lose their
    /// leaves each winter.
    DeciduousForest,
    /// Coniferous forests are made up of coniferous or cone-bearing trees, most of which are evergreens.
    ConiferousForest,
    /// Characterized by coniferous forests consisting mostly of pines, spruces, and larches.
    Taiga,
    /// Land covered with woody plants, mainly trees, and shrubs.
    Woodland,
    /// Plant community dominated by shrubs, often with small or no trees.
    Shrubland,
    /// Savanna is a type of grassland with scattered trees.
    Savanna,
    /// Grassland is a type of land with grass and very few trees.
    Grassland,
    /// A biome where tree growth is hindered by frigid temperatures and short growing seasons.
    Tundra,
    /// A biome where little precipitation occurs and consequently living conditions are hostile for plant and
    /// animal life.
    Desert,
    /// Plant community dominated by grasses, often with small or no shrubs.
    Scrub,
    /// A wetland with spongy ground and a lot of moss.
    Bog,
    /// A wetland that features permanent inundation of large areas of land by shallow bodies of water, generally
    /// with substantial tree cover.
    FreshwaterSwamp,
    /// A wetland that features permanent inundation of large areas of land by shallow bodies of water, generally
    /// with little tree cover. This one has saltwater.
    SaltwaterSwamp,
    /// An area of coastal grassland that is frequently flooded by seawater.
    Saltmarsh,
    /// A wetland that is dominated by herbaceous rather than woody plant species.
    Wetland,
    /// A large body of water, either fresh or salt, that is surrounded by land.
    LargeLake,
    /// A river delta is a landform shaped like a triangle, created by the deposition of sediment that is carried
    /// by a river and enters slower-moving or stagnant water. This occurs at a river mouth, when it enters an ocean,
    /// sea, estuary, lake, reservoir, or another river that cannot carry away the supplied sediment.
    RiverDelta,
    /// Rivers that flow into the ocean.
    CoastalRiver,
    /// Rivers that flow into a lake.
    InlandRiver,
    /// An area of low-lying ground adjacent to a river, formed mainly of river sediments and subject to flooding.
    Floodplain,
    /// A fast-flowing river, typically with a steep gradient and a rocky bed.
    UplandRiver,
    /// An endorheic basin is a drainage basin that normally retains water and allows no outflow to other, external
    /// bodies of water; instead, the water drainage flows into permanent and seasonal lakes and swamps that equilibrate
    /// through evaporation.
    EndorheicBasin,
    /// An island that rises to the ocean surface from the ocean floor.
    OceanicIsland,
    /// A very large body of salt water. Forms beaches next to land, but can also be found in the middle of the ocean.
    Sea,
    /// A large body of salt water that is extremely deep. This is not found near land.
    DeepSea,
    /// A large body of salt water with a flat bottom. This is not found near land.
    SeaShelf,
    /// An area of sea with a lot of coral reefs and other marine life.
    CoralReef,
    /// An area of sea that has a lot of kelp. Supports a lot of marine life.
    KelpForest,
    /// A persistent body of dense ice that is constantly moving under its own weight. A glacier forms where the accumulation
    /// of snow exceeds its ablation over many years, often centuries.
    Glacier,
    /// A very large sheet of ice that permanently covers the land.
    IceSheet,
    /// Nothing
    #[default]
    Barren,
    /// A biome that represents the inside of a building.
    Indoor,
    /// A cityscape or other urban development.
    Urban,
    /// Land that has been cleared of trees for farmland.
    Cultivated,
    /// Forested land that is used for logging or has otherwise been planted intentionally.
    Forested,
    /// Land that has been cleared of trees and is used for grazing animals.
    Rangeland,
}

/// Humidity of the biome. These are descriptions humidity provinces of the biome.
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
pub enum Humidity {
    /// Extremely dry
    Superarid,
    /// Very dry
    Perarid,
    /// Dry
    Arid,
    /// Somewhat dry
    #[default]
    Semiarid,
    /// Somewhat wet
    Subhumid,
    /// Wet
    Humid,
    /// Very wet
    Perhumid,
    /// Extremely wet
    Superhumid,
}

/// Altitude of the biome (altitudinal belts). These are descriptions of the
/// biotemperature of the biome.
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
pub enum Altitude {
    /// Cooler than 1.5 Celsius
    Alvar,
    /// Within 1.5-3 Celsius
    Alpine,
    /// Within 3-6 Celsius
    Subalpine,
    /// Within 6-12 Celsius
    #[default]
    Montane,
    /// Within 12-24 Celsius
    LowerMontane,
    /// Warmer than 24 Celsius
    Premontane,
}

/// Describes the latitudinal regions of the biome (temperature bands)
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
pub enum Latitude {
    /// Close to the equator
    Tropical,
    /// 2nd closest to the equator
    Subtropical,
    /// 3rd closest to the equator
    #[default]
    WarmTemperate,
    /// 4th closest to the equator and poles
    CoolTemperate,
    /// 3rd closest to the poles
    Boreal,
    /// Close to the poles
    Subpolar,
    /// Closest to the poles
    Polar,
}
