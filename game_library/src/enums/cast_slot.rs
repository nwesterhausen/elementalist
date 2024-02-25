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
    /// Cantrips are typically basic attacks that don't cost mana and are a player's primary means
    /// of dealing damage. They are typically the most spammable spell in a player's arsenal.
    Cantrip,
    /// Slotted spells are the primary spells a player uses. They are typically more powerful than
    /// cantrips and have a variety of effects.
    Slot,
    /// Ultimate spells are not in the player's spellbook, instead they are learned organically
    /// while in the "primal realm." They are typically very powerful and have a long cooldown.
    Ultimate,
}

impl std::fmt::Display for CastSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cantrip => write!(f, "Cantrip"),
            Self::Slot => write!(f, "Slot"),
            Self::Ultimate => write!(f, "Ultimate"),
        }
    }
}
