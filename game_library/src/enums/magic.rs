use bevy::ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::Skill;

/// The different types of magic.
///
/// Each school of magic has a corresponding [`Skill`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Resource, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MagicType {
    /// Fire magic, pyromancy
    Fire,
    /// Lightning magic, fulgamancy
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
        use MagicType::*;
        [
            Fire,
            Lightning,
            Water,
            Earth,
            Air,
            Ice,
            Force,
            Light,
            Dark,
            Arcane,
            Life,
            Death,
            Enhancement,
            Reduction,
            Summoning,
            Necromancy,
            Polymorph,
            Time,
        ]
        .iter()
        .copied()
    }
    /// Returns the corresponding [`Skill`] for this [`MagicType`]
    pub fn skill(self) -> Skill {
        use MagicType::*;
        match self {
            Fire => Skill::Pyromancy,
            Lightning => Skill::Fulgamancy,
            Water => Skill::Hydromancy,
            Earth => Skill::Geomancy,
            Air => Skill::Aeromancy,
            Ice => Skill::Cryomancy,
            Force => Skill::Trudomancy,
            Light => Skill::Photomancy,
            Dark => Skill::Umbramancy,
            Arcane => Skill::Arcanomancy,
            Life => Skill::Vitomancy,
            Death => Skill::Mortomancy,
            Enhancement => Skill::Ampiliomancy,
            Reduction => Skill::Diminiomancy,
            Summoning => Skill::Citomancy,
            Necromancy => Skill::Necromancy,
            Polymorph => Skill::Mutatiomancy,
            Time => Skill::Chronomancy,
        }
    }
    /// Returns the corresponding [`MagicType`] for this [`Skill`] if it exists
    pub fn from_skill(skill: Skill) -> Option<Self> {
        use MagicType::*;
        match skill {
            Skill::Pyromancy => Some(Fire),
            Skill::Fulgamancy => Some(Lightning),
            Skill::Hydromancy => Some(Water),
            Skill::Geomancy => Some(Earth),
            Skill::Aeromancy => Some(Air),
            Skill::Cryomancy => Some(Ice),
            Skill::Trudomancy => Some(Force),
            Skill::Photomancy => Some(Light),
            Skill::Umbramancy => Some(Dark),
            Skill::Arcanomancy => Some(Arcane),
            Skill::Vitomancy => Some(Life),
            Skill::Mortomancy => Some(Death),
            Skill::Ampiliomancy => Some(Enhancement),
            Skill::Diminiomancy => Some(Reduction),
            Skill::Citomancy => Some(Summoning),
            Skill::Necromancy => Some(Necromancy),
            Skill::Mutatiomancy => Some(Polymorph),
            Skill::Chronomancy => Some(Time),
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
        use MagicType::*;
        match self {
            Fire => write!(f, "Fire"),
            Lightning => write!(f, "Lightning"),
            Water => write!(f, "Water"),
            Earth => write!(f, "Earth"),
            Air => write!(f, "Air"),
            Ice => write!(f, "Ice"),
            Force => write!(f, "Force"),
            Light => write!(f, "Light"),
            Dark => write!(f, "Dark"),
            Arcane => write!(f, "Arcane"),
            Life => write!(f, "Life"),
            Death => write!(f, "Death"),
            Enhancement => write!(f, "Enhancement"),
            Reduction => write!(f, "Reduction"),
            Summoning => write!(f, "Summoning"),
            Necromancy => write!(f, "Necromancy"),
            Polymorph => write!(f, "Polymorph"),
            Time => write!(f, "Time"),
        }
    }
}
