//! This file contains the spell bundle, which is a bundle of components that are needed for a spell.
//!
//! This bundle contains the movement bundle, the sprite sheet bundle, and the spell lifetime component.

use crate::{MovementBundle, SpellLifetime};
use bevy::prelude::*;

/// Bundle that contains all components needed for a spell
///
/// This has the information to move the spell, display it, and despawn it when it's lifetime expires.
#[derive(Bundle)]
pub struct SpellBundle {
    /// The movement bundle (velocity and acceleration)
    ///
    /// The speed of the spell should be affected by the player's stats.
    pub movement: MovementBundle,
    /// The sprite sheet bundle (sprite sheet and transform)
    pub sprite: SpriteSheetBundle,
    /// The spell lifetime component (how long the spell should last)
    ///
    /// This should be affected by the player's stats.
    pub lifetime: SpellLifetime,
}
