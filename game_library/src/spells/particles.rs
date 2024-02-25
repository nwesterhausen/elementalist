//! Particle effects for spells.
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::enums::ParticleAttachment;

/// A particle that a spell can create.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, Component, Resource,
)]
#[serde(rename_all = "camelCase")]
pub struct SpellParticles {
    /// The unique_id for the particle effect
    pub particle_id: String,
    /// The attachment point for the particle effect
    pub attachment: ParticleAttachment,
}
