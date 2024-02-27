//! Animation definitions for entity sprites.
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// The different animation states for an entity sprite.
///
/// Not every entity sprite will have all of these states. Any left undefined in the basic definition will
/// fall back to the default state (idle).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Reflect,
    Serialize,
    Deserialize,
    Component,
    Resource,
    Default,
)]
pub enum Animation {
    /// The default state, typically standing still.
    #[default]
    Idle,
    /// The walking state.
    Walk,
    /// The running state.
    Run,
    /// The jumping state.
    Jump,
    /// The spell-casting state.
    Cast,
    /// The melee attack state.
    Attack,
    /// A dying state
    Death,
    /// A state for getting hurt
    Hurt,
    /// A state for being stunned
    Stun,
    /// A state for being dead.
    Dead,
    // Combinations for walk + other
    /// The walking and spell-casting state.
    WalkCast,
    /// The walking and melee attack state.
    WalkAttack,
    // Combinations for run + other
    /// The running and spell-casting state.
    RunCast,
    /// The running and melee attack state.
    RunAttack,
}

/// An animation definition for an entity sprite.
#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, Component, Resource)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct AnimationDefinition {
    /// The animation state.
    pub state: Animation,
    /// The tileset to use for the animation (by unique_id)
    pub tileset: String,
    /// The tile indices for the animation.
    pub tile_indices: Vec<usize>,
    /// The duration of each frame in the animation.
    pub frame_duration: f32,
}

impl Hash for AnimationDefinition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.state.hash(state);
        self.tileset.hash(state);
        self.tile_indices.hash(state);
    }
}

impl Default for AnimationDefinition {
    fn default() -> Self {
        Self {
            state: Animation::Idle,
            tileset: String::from("placeholder"),
            tile_indices: vec![0],
            frame_duration: 0.0,
        }
    }
}
