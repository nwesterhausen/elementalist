//! Definition for an entity's sprite.
//!
//! This is used to define the sprite for an entity. This is used in the game system to display the entity's sprite
//! and to handle collision with the entity. It includes what tileset to use, and the tile indices for any sprite
//! animations.
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::InternalId;

use super::{Animation, AnimationDefinition};

/// The sprite for an entity.
#[derive(Debug, Clone, PartialEq, Hash, Reflect, Serialize, Deserialize, Component, Resource)]
#[serde(rename_all = "camelCase")]
pub struct EntitySprite {
    /// The unique id of the entity sprite.
    #[serde(default = "String::new")]
    unique_id: String,
    /// The animation definitions for the entity sprite.
    animations: Vec<AnimationDefinition>,
}

impl EntitySprite {
    /// Create a new empty entity sprite.
    #[must_use]
    pub fn new(unique_id: &str, idle_animation: AnimationDefinition) -> Self {
        Self {
            unique_id: unique_id.to_string(),
            animations: vec![idle_animation],
        }
    }

    /// Add an animation definition to the entity sprite.
    pub fn add_animation(&mut self, animation: AnimationDefinition) {
        // Check if the animation already exists
        for a in &mut self.animations {
            if a.state == animation.state {
                info!(
                    "Updating animation: {:?} in entity sprite {}",
                    animation.state, self.unique_id
                );
                *a = animation;
                return;
            }
        }
        // If it doesn't exist, add it
        self.animations.push(animation);
    }

    /// Get the animation definition for the given state.
    ///
    /// If it doesn't exist, return the default animation.
    #[must_use]
    pub fn get_animation(&self, state: Animation) -> AnimationDefinition {
        for a in &self.animations {
            if a.state == state {
                return a.clone();
            }
        }
        // If the animation doesn't exist, return the default animation
        warn!(
            "Animation {:?} not found in entity sprite {}. Returning default animation.",
            state, self.unique_id
        );

        // The default idle animation should always exist, but in case it doesn't, return a default animation
        self.animations.first().map_or_else(
            || {
                error!(
                    "No default animation found for entity sprite {}. This should never happen.",
                    self.unique_id
                );
                AnimationDefinition::default()
            },
            std::clone::Clone::clone,
        )
    }

    /// Get the unique id of the entity sprite.
    #[must_use]
    pub fn unique_id(&self) -> &str {
        &self.unique_id
    }

    /// Get the animation definitions for the entity sprite.
    #[must_use]
    pub const fn animations(&self) -> &Vec<AnimationDefinition> {
        &self.animations
    }
}

impl InternalId for EntitySprite {
    fn get_internal_id(&self) -> String {
        self.unique_id.clone()
    }

    fn update_internal_id(&mut self) {}
}

impl Default for EntitySprite {
    fn default() -> Self {
        Self {
            unique_id: String::from("non-existent-entity-sprite"),
            animations: vec![AnimationDefinition::default()],
        }
    }
}
