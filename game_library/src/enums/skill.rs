use bevy::{
    ecs::{component::Component, system::Resource},
    reflect::Reflect,
};
use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Serialize};

use crate::enums::MagicType;

/// The different skills. Each skill is a school of magic.
///
/// It's possible we end up adding a few non-magic skills, but for now
/// it's just what corresponds to the different spells so far.
///
/// Skills are used to track a meta-progression of a player's abilities.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Component,
    Resource,
    Serialize,
    Deserialize,
    Reflect,
    InspectorOptions,
)]
#[serde(rename_all = "camelCase")]
pub enum Skill {
    /// Pyromancy is the school of fire magic.
    Pyromancy,
    /// Fulgomancy is the school of lightning magic.
    Fulgomancy,
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
        [
            Self::Pyromancy,
            Self::Fulgomancy,
            Self::Hydromancy,
            Self::Geomancy,
            Self::Aeromancy,
            Self::Cryomancy,
            Self::Trudomancy,
            Self::Photomancy,
            Self::Umbramancy,
            Self::Arcanomancy,
            Self::Vitomancy,
            Self::Mortomancy,
            Self::Ampiliomancy,
            Self::Diminiomancy,
            Self::Citomancy,
            Self::Necromancy,
            Self::Mutatiomancy,
            Self::Chronomancy,
        ]
        .iter()
        .copied()
    }
    /// Returns the corresponding [`MagicType`] for this [`Skill`]
    #[must_use]
    pub const fn magic_type(self) -> MagicType {
        match self {
            Self::Pyromancy => MagicType::Fire,
            Self::Fulgomancy => MagicType::Lightning,
            Self::Hydromancy => MagicType::Water,
            Self::Geomancy => MagicType::Earth,
            Self::Aeromancy => MagicType::Air,
            Self::Cryomancy => MagicType::Ice,
            Self::Trudomancy => MagicType::Force,
            Self::Photomancy => MagicType::Light,
            Self::Umbramancy => MagicType::Dark,
            Self::Arcanomancy => MagicType::Arcane,
            Self::Vitomancy => MagicType::Life,
            Self::Mortomancy => MagicType::Death,
            Self::Ampiliomancy => MagicType::Enhancement,
            Self::Diminiomancy => MagicType::Reduction,
            Self::Citomancy => MagicType::Summoning,
            Self::Necromancy => MagicType::Necromancy,
            Self::Mutatiomancy => MagicType::Polymorph,
            Self::Chronomancy => MagicType::Time,
        }
    }

    /// Get the index of the skill icon in the tile atlas
    ///
    /// This is hard-coded for now, but we could make it configurable..
    #[must_use]
    pub const fn icon_index(self) -> usize {
        match self {
            Self::Aeromancy => 0,
            Self::Arcanomancy => 1,
            Self::Ampiliomancy => 2,
            Self::Chronomancy => 3,
            Self::Citomancy => 4,
            Self::Cryomancy => 5,
            Self::Diminiomancy => 6,
            Self::Fulgomancy => 7,
            Self::Geomancy => 8,
            Self::Hydromancy => 9,
            Self::Mortomancy => 10,
            Self::Mutatiomancy => 11,
            Self::Necromancy => 12,
            Self::Photomancy => 13,
            Self::Pyromancy => 14,
            Self::Trudomancy => 15,
            Self::Umbramancy => 16,
            Self::Vitomancy => 17,
        }
    }
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Pyromancy => "Pyromancy",
            Self::Fulgomancy => "Fulgomancy",
            Self::Hydromancy => "Hydromancy",
            Self::Geomancy => "Geomancy",
            Self::Aeromancy => "Aeromancy",
            Self::Cryomancy => "Cryomancy",
            Self::Trudomancy => "Trudomancy",
            Self::Photomancy => "Photomancy",
            Self::Umbramancy => "Umbramancy",
            Self::Arcanomancy => "Arcanomancy",
            Self::Vitomancy => "Vitomancy",
            Self::Mortomancy => "Mortomancy",
            Self::Ampiliomancy => "Ampiliomancy",
            Self::Diminiomancy => "Diminiomancy",
            Self::Citomancy => "Citomancy",
            Self::Necromancy => "Necromancy",
            Self::Mutatiomancy => "Mutatiomancy",
            Self::Chronomancy => "Chronomancy",
        };
        write!(f, "{}", s)
    }
}
