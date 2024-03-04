//! This module contains the different states that an entity can be in. These states are used to determine the final animation state.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The different movement states for an entity. This is used to determine the final animation state.
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize, Component, Resource,
)]
pub enum Movement {
    /// The entity is idle.
    #[default]
    Idle,
    /// The entity is walking.
    Walk,
    /// The entity is running.
    Run,
    /// The entity is jumping.
    Jump,
}

/// The different action states for an entity. This is used to determine the final animation state.
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize, Component, Resource,
)]
pub enum Action {
    /// The entity is not taking any action.
    #[default]
    Idle,
    /// The entity is casting a spell.
    Cast,
    /// The entity is attacking.
    Attack,
}

/// The different reaction states for an entity. This is used to determine the final animation state.
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize, Component, Resource,
)]
pub enum Reaction {
    /// The entity is not reacting to anything.
    #[default]
    None,
    /// The entity is hurt.
    Hurt,
    /// The entity is stunned.
    Stun,
    /// The entity is dead.
    Dead,
}
