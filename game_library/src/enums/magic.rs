use bevy::ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::enums::Skill;

/// The different types of magic.
///
/// Each school of magic has a corresponding [`Skill`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Resource, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub enum MagicType {
    /// Fire magic, pyromancy
    Fire,
    /// Lightning magic, fulgomancy
    Lightning,
    /// Water magic, hydromancy
    Water,
    /// Earth magic, geomancy
    Earth,
    /// Air magic, aeromancy
    Air,
    /// Ice magic, cryomancy
    Ice,
    /// Force magic, trudomancy
    Force,
    /// Light magic, photomancy
    Light,
    /// Dark magic, umbramancy
    Dark,
    /// Arcane magic, arcanomancy
    Arcane,
    /// Life magic, vitomancy
    Life,
    /// Death magic, mortomancy
    Death,
    /// Enhancement magic, ampiliomancy
    Enhancement,
    /// Reduction magic, diminiomancy
    Reduction,
    /// Summoning magic, citomancy
    Summoning,
    /// Necromancy magic, necromancy
    Necromancy,
    /// Polymorph magic, mutatiomancy
    Polymorph,
    /// Time magic, chronomancy
    Time,
}

impl MagicType {
    /// Returns an iterator over all the variants of `MagicType`
    pub fn variants() -> impl Iterator<Item = Self> {
        [
            MagicType::Fire,
            MagicType::Lightning,
            MagicType::Water,
            MagicType::Earth,
            MagicType::Air,
            MagicType::Ice,
            MagicType::Force,
            MagicType::Light,
            MagicType::Dark,
            MagicType::Arcane,
            MagicType::Life,
            MagicType::Death,
            MagicType::Enhancement,
            MagicType::Reduction,
            MagicType::Summoning,
            MagicType::Necromancy,
            MagicType::Polymorph,
            MagicType::Time,
        ]
        .iter()
        .copied()
    }
    /// Returns the corresponding [`Skill`] for this [`MagicType`]
    #[must_use]
    pub fn skill(self) -> Skill {
        match self {
            MagicType::Fire => Skill::Pyromancy,
            MagicType::Lightning => Skill::Fulgomancy,
            MagicType::Water => Skill::Hydromancy,
            MagicType::Earth => Skill::Geomancy,
            MagicType::Air => Skill::Aeromancy,
            MagicType::Ice => Skill::Cryomancy,
            MagicType::Force => Skill::Trudomancy,
            MagicType::Light => Skill::Photomancy,
            MagicType::Dark => Skill::Umbramancy,
            MagicType::Arcane => Skill::Arcanomancy,
            MagicType::Life => Skill::Vitomancy,
            MagicType::Death => Skill::Mortomancy,
            MagicType::Enhancement => Skill::Ampiliomancy,
            MagicType::Reduction => Skill::Diminiomancy,
            MagicType::Summoning => Skill::Citomancy,
            MagicType::Necromancy => Skill::Necromancy,
            MagicType::Polymorph => Skill::Mutatiomancy,
            MagicType::Time => Skill::Chronomancy,
        }
    }
    /// Returns the corresponding [`MagicType`] for this [`Skill`] if it exists
    #[must_use]
    pub fn from_skill(skill: Skill) -> Option<Self> {
        match skill {
            Skill::Pyromancy => Some(MagicType::Fire),
            Skill::Fulgomancy => Some(MagicType::Lightning),
            Skill::Hydromancy => Some(MagicType::Water),
            Skill::Geomancy => Some(MagicType::Earth),
            Skill::Aeromancy => Some(MagicType::Air),
            Skill::Cryomancy => Some(MagicType::Ice),
            Skill::Trudomancy => Some(MagicType::Force),
            Skill::Photomancy => Some(MagicType::Light),
            Skill::Umbramancy => Some(MagicType::Dark),
            Skill::Arcanomancy => Some(MagicType::Arcane),
            Skill::Vitomancy => Some(MagicType::Life),
            Skill::Mortomancy => Some(MagicType::Death),
            Skill::Ampiliomancy => Some(MagicType::Enhancement),
            Skill::Diminiomancy => Some(MagicType::Reduction),
            Skill::Citomancy => Some(MagicType::Summoning),
            Skill::Necromancy => Some(MagicType::Necromancy),
            Skill::Mutatiomancy => Some(MagicType::Polymorph),
            Skill::Chronomancy => Some(MagicType::Time),
        }
    }
}

impl From<MagicType> for Skill {
    fn from(magic: MagicType) -> Self {
        magic.skill()
    }
}

impl From<Skill> for MagicType {
    /// Returns the corresponding [`MagicType`] for this [`Skill`] if it exists
    ///
    /// This will return [`MagicType::Arcane`] if the [`Skill`] is not a magic skill
    fn from(skill: Skill) -> Self {
        MagicType::from_skill(skill).unwrap_or(MagicType::Arcane)
    }
}

impl std::fmt::Display for MagicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MagicType::Fire => write!(f, "Fire"),
            MagicType::Lightning => write!(f, "Lightning"),
            MagicType::Water => write!(f, "Water"),
            MagicType::Earth => write!(f, "Earth"),
            MagicType::Air => write!(f, "Air"),
            MagicType::Ice => write!(f, "Ice"),
            MagicType::Force => write!(f, "Force"),
            MagicType::Light => write!(f, "Light"),
            MagicType::Dark => write!(f, "Dark"),
            MagicType::Arcane => write!(f, "Arcane"),
            MagicType::Life => write!(f, "Life"),
            MagicType::Death => write!(f, "Death"),
            MagicType::Enhancement => write!(f, "Enhancement"),
            MagicType::Reduction => write!(f, "Reduction"),
            MagicType::Summoning => write!(f, "Summoning"),
            MagicType::Necromancy => write!(f, "Necromancy"),
            MagicType::Polymorph => write!(f, "Polymorph"),
            MagicType::Time => write!(f, "Time"),
        }
    }
}
