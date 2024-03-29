use bevy::{
    ecs::{component::Component, system::Resource},
    reflect::Reflect,
};
use serde::{Deserialize, Serialize};

/// Where the spell can be slotted for casting.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Resource, Serialize, Deserialize, Reflect,
)]
#[serde(rename_all = "camelCase")]
pub enum CastSlot {
    /// Primary spells typically have no mana cost and a short cooldown. Typically used for basic
    /// attacks.
    Primary,
    /// Secondary spells will have a mana cost (usually) and can range from really powerful to
    /// other kinds of spells. Typically used for something that is not a basic attack.
    Secondary,
    /// Defensive spells are typically used to defend against attacks or to heal. The common
    /// type of spell used for this slot is a shield or barrier. Other things might generate a
    /// wall or other obstacle.
    Defensive,
    /// Ultimate spells are not in the player's spellbook, instead they are learned organically
    /// while in the "primal realm." They are typically very powerful and have a long cooldown.
    Ultimate,
}

impl std::fmt::Display for CastSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primary => write!(f, "Primary"),
            Self::Secondary => write!(f, "Secondary"),
            Self::Defensive => write!(f, "Defensive"),
            Self::Ultimate => write!(f, "Ultimate"),
        }
    }
}
