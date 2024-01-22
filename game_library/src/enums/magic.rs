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
            Self::Fire,
            Self::Lightning,
            Self::Water,
            Self::Earth,
            Self::Air,
            Self::Ice,
            Self::Force,
            Self::Light,
            Self::Dark,
            Self::Arcane,
            Self::Life,
            Self::Death,
            Self::Enhancement,
            Self::Reduction,
            Self::Summoning,
            Self::Necromancy,
            Self::Polymorph,
            Self::Time,
        ]
        .iter()
        .copied()
    }
    /// Returns the corresponding [`Skill`] for this [`MagicType`]
    #[must_use]
    pub const fn skill(self) -> Skill {
        match self {
            Self::Fire => Skill::Pyromancy,
            Self::Lightning => Skill::Fulgomancy,
            Self::Water => Skill::Hydromancy,
            Self::Earth => Skill::Geomancy,
            Self::Air => Skill::Aeromancy,
            Self::Ice => Skill::Cryomancy,
            Self::Force => Skill::Trudomancy,
            Self::Light => Skill::Photomancy,
            Self::Dark => Skill::Umbramancy,
            Self::Arcane => Skill::Arcanomancy,
            Self::Life => Skill::Vitomancy,
            Self::Death => Skill::Mortomancy,
            Self::Enhancement => Skill::Ampiliomancy,
            Self::Reduction => Skill::Diminiomancy,
            Self::Summoning => Skill::Citomancy,
            Self::Necromancy => Skill::Necromancy,
            Self::Polymorph => Skill::Mutatiomancy,
            Self::Time => Skill::Chronomancy,
        }
    }
    /// Returns the corresponding [`MagicType`] for this [`Skill`] if it exists
    #[must_use]
    pub const fn from_skill(skill: Skill) -> Option<Self> {
        match skill {
            Skill::Pyromancy => Some(Self::Fire),
            Skill::Fulgomancy => Some(Self::Lightning),
            Skill::Hydromancy => Some(Self::Water),
            Skill::Geomancy => Some(Self::Earth),
            Skill::Aeromancy => Some(Self::Air),
            Skill::Cryomancy => Some(Self::Ice),
            Skill::Trudomancy => Some(Self::Force),
            Skill::Photomancy => Some(Self::Light),
            Skill::Umbramancy => Some(Self::Dark),
            Skill::Arcanomancy => Some(Self::Arcane),
            Skill::Vitomancy => Some(Self::Life),
            Skill::Mortomancy => Some(Self::Death),
            Skill::Ampiliomancy => Some(Self::Enhancement),
            Skill::Diminiomancy => Some(Self::Reduction),
            Skill::Citomancy => Some(Self::Summoning),
            Skill::Necromancy => Some(Self::Necromancy),
            Skill::Mutatiomancy => Some(Self::Polymorph),
            Skill::Chronomancy => Some(Self::Time),
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
        Self::from_skill(skill).unwrap_or(Self::Arcane)
    }
}

impl std::fmt::Display for MagicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fire => write!(f, "Fire"),
            Self::Lightning => write!(f, "Lightning"),
            Self::Water => write!(f, "Water"),
            Self::Earth => write!(f, "Earth"),
            Self::Air => write!(f, "Air"),
            Self::Ice => write!(f, "Ice"),
            Self::Force => write!(f, "Force"),
            Self::Light => write!(f, "Light"),
            Self::Dark => write!(f, "Dark"),
            Self::Arcane => write!(f, "Arcane"),
            Self::Life => write!(f, "Life"),
            Self::Death => write!(f, "Death"),
            Self::Enhancement => write!(f, "Enhancement"),
            Self::Reduction => write!(f, "Reduction"),
            Self::Summoning => write!(f, "Summoning"),
            Self::Necromancy => write!(f, "Necromancy"),
            Self::Polymorph => write!(f, "Polymorph"),
            Self::Time => write!(f, "Time"),
        }
    }
}
