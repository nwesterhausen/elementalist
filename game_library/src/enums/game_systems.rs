use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

/// The game systems which can be defined using data files.
/// These are not every system in the game, but any system determined to be "modular"
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Reflect)]
pub enum GameSystem {
    /// The spell system (i.e. the data file is a spell data file)
    Spell,
    /// The tileset system is responsible for loading tilesets as sprite atlases
    Tileset,
    /// A particle effect to use in the game
    Particle,
    /// The realm system is responsible for loading realms
    Realm,
}

/// The order in which the game systems should be loaded.
///
/// Since some data files depend on others, we need to load them in a specific order.
///
/// 1. Tilesets are loaded. This is because spells may reference tilesets.
/// 2. Spells are loaded.
///
/// Upcoming systems that would influence this list:
///
/// - Sprites
/// - Skill perks / trees
/// - Monsters
#[allow(dead_code)]
pub const ORDERED: [GameSystem; 4] = [
    GameSystem::Tileset,
    GameSystem::Realm,
    GameSystem::Particle,
    GameSystem::Spell,
];
