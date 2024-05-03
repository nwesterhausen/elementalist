//! Definitions and systems pertaining to the polymorph effect.
//!
//! The polymorph effect is a spell effect that transforms the target into a different entity. In the game system,
//! this is handled by changing:
//!
//! - The entity's sprite
//! - The entity's collision
//! - The entity's stats
//! - Tracking the time left for the polymorph effect
//!
//! After the time has expired, the entity is transformed back into its original form.
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::StatEffect;

/// A polymorph entity. This is a shell for tagging entities that are polymorphed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize, Component)]
#[allow(clippy::module_name_repetitions)]
pub struct Polymorphed(pub String);

/// Stat changes for the polymorph effect.
#[derive(Debug, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize, Component)]
#[allow(clippy::module_name_repetitions)]
pub struct PolymorphStatChange {
    /// Stats which replace the original entity's stats.
    pub replace: Vec<StatEffect>,
    /// Stats which are added to the original entity's stats.
    pub add: Vec<StatEffect>,
    /// Stats which are multiplied to the original entity's stats.
    pub multiply: Vec<StatEffect>,
    /// Stats which are subtracted from the original entity's stats.
    pub subtract: Vec<StatEffect>,
    /// Stats which the original entity's stats are divided by.
    pub divide: Vec<StatEffect>,
}

/// Define the details of a polymorph effect.
#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct PolymorphEffect {
    /// The id of this polymorph effect.
    pub unique_id: String,
    /// The duration of the polymorph effect.
    pub duration: f32,
    /// The name of the polymorph effect. (This is used as the new entity's name.)
    pub name: String,
    /// The entity_sprite to use for the polymorphed entity (by unique_id).
    pub polymorph_sprite: String,
    /// The stat changes for the polymorph effect.
    pub stat_changes: PolymorphStatChange,
}
