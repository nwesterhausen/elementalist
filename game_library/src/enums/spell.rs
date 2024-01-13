use bevy::ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::{MagicType, Skill};

/// The different spells that can be cast
///
/// Spells are cast using the `CastSpell` event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Resource, Serialize, Deserialize)]
pub enum Spell {
    /// The firebolt spell
    ///
    /// - [`MagicType::Fire`]
    /// - [`Skill::Pyromancy`]
    Firebolt,
    /// The stone dart spell
    ///
    /// - [`MagicType::Earth`]
    /// - [`Skill::Geomancy`]
    StoneDart,
}

impl Spell {
    /// Returns an iterator over all the variants of `Spell` (except `Empty`)
    pub fn variants() -> impl Iterator<Item = Self> {
        use Spell::*;
        [Firebolt, StoneDart].iter().copied()
    }
    /// Returns the [`MagicType`] of the spell
    pub fn magic_type(&self) -> MagicType {
        match self {
            Spell::Firebolt => MagicType::Fire,
            Spell::StoneDart => MagicType::Earth,
        }
    }
    /// Returns the [`Skill`] of the spell
    pub fn skill(&self) -> Skill {
        match self {
            Spell::Firebolt => Skill::Pyromancy,
            Spell::StoneDart => Skill::Geomancy,
        }
    }
}
