use bevy::ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::MagicType;

/// The different skills. Each skill is a school of magic.
///
/// It's possible we end up adding a few non-magic skills, but for now
/// it's just what corresponds to the different spells so far.
///
/// Skills are used to track a meta-progression of a player's abilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Resource, Serialize, Deserialize)]
pub enum Skill {
    /// Pyromancy is the school of fire magic.
    Pyromancy,
    /// Fulgamancy is the school of lightning magic.
    Fulgamancy,
    /// Hydromancy is the school of water magic.
    Hydromancy,
    /// Geomancy is the school of earth magic.
    Geomancy,
    /// Aeromancy is the school of air magic.
    Aeromancy,
    /// Cryomancy is the school of ice magic.
    Cryomancy,
    /// Trudomancy is the school of force magic.
    Trudomancy,
    /// Photomancy is the school of light magic.
    Photomancy,
    /// Umbramancy is the school of dark magic.
    Umbramancy,
    /// Arcanomancy is the school of arcane magic.
    Arcanomancy,
    /// Vitomancy is the school of life magic.
    Vitomancy,
    /// Mortomancy is the school of death magic.
    Mortomancy,
    /// Ampiliomancy is the school of enhancement magic.
    Ampiliomancy,
    /// Diminiomancy is the school of reduction magic.
    Diminiomancy,
    /// Citomancy is the school of summoning magic.
    Citomancy,
    /// Necromancy is the school of necromancy.
    Necromancy,
    /// Mutatiomancy is the school of polymorph magic.
    Mutatiomancy,
    /// Chronomancy is the school of time magic.
    Chronomancy,
}

impl Skill {
    /// Returns an iterator over all the variants of `Skill`
    pub fn variants() -> impl Iterator<Item = Self> {
        use Skill::*;
        [
            Pyromancy,
            Fulgamancy,
            Hydromancy,
            Geomancy,
            Aeromancy,
            Cryomancy,
            Trudomancy,
            Photomancy,
            Umbramancy,
            Arcanomancy,
            Vitomancy,
            Mortomancy,
            Ampiliomancy,
            Diminiomancy,
            Citomancy,
            Necromancy,
            Mutatiomancy,
            Chronomancy,
        ]
        .iter()
        .copied()
    }
    /// Returns the corresponding [`MagicType`] for this [`Skill`]
    pub fn magic_type(self) -> MagicType {
        use Skill::*;
        match self {
            Pyromancy => MagicType::Fire,
            Fulgamancy => MagicType::Lightning,
            Hydromancy => MagicType::Water,
            Geomancy => MagicType::Earth,
            Aeromancy => MagicType::Air,
            Cryomancy => MagicType::Ice,
            Trudomancy => MagicType::Force,
            Photomancy => MagicType::Light,
            Umbramancy => MagicType::Dark,
            Arcanomancy => MagicType::Arcane,
            Vitomancy => MagicType::Life,
            Mortomancy => MagicType::Death,
            Ampiliomancy => MagicType::Enhancement,
            Diminiomancy => MagicType::Reduction,
            Citomancy => MagicType::Summoning,
            Necromancy => MagicType::Necromancy,
            Mutatiomancy => MagicType::Polymorph,
            Chronomancy => MagicType::Time,
        }
    }
}
