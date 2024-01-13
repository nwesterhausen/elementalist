use bevy::ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::Skill;

/// The different types of magic.
///
/// Each school of magic has a corresponding [`Skill`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Resource, Serialize, Deserialize)]
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
