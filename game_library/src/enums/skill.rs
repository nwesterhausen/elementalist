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
            Skill::Pyromancy,
            Skill::Fulgomancy,
            Skill::Hydromancy,
            Skill::Geomancy,
            Skill::Aeromancy,
            Skill::Cryomancy,
            Skill::Trudomancy,
            Skill::Photomancy,
            Skill::Umbramancy,
            Skill::Arcanomancy,
            Skill::Vitomancy,
            Skill::Mortomancy,
            Skill::Ampiliomancy,
            Skill::Diminiomancy,
            Skill::Citomancy,
            Skill::Necromancy,
            Skill::Mutatiomancy,
            Skill::Chronomancy,
        ]
        .iter()
        .copied()
    }
    /// Returns the corresponding [`MagicType`] for this [`Skill`]
    #[must_use]
    pub fn magic_type(self) -> MagicType {
        match self {
            Skill::Pyromancy => MagicType::Fire,
            Skill::Fulgomancy => MagicType::Lightning,
            Skill::Hydromancy => MagicType::Water,
            Skill::Geomancy => MagicType::Earth,
            Skill::Aeromancy => MagicType::Air,
            Skill::Cryomancy => MagicType::Ice,
            Skill::Trudomancy => MagicType::Force,
            Skill::Photomancy => MagicType::Light,
            Skill::Umbramancy => MagicType::Dark,
            Skill::Arcanomancy => MagicType::Arcane,
            Skill::Vitomancy => MagicType::Life,
            Skill::Mortomancy => MagicType::Death,
            Skill::Ampiliomancy => MagicType::Enhancement,
            Skill::Diminiomancy => MagicType::Reduction,
            Skill::Citomancy => MagicType::Summoning,
            Skill::Necromancy => MagicType::Necromancy,
            Skill::Mutatiomancy => MagicType::Polymorph,
            Skill::Chronomancy => MagicType::Time,
        }
    }
}
