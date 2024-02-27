//! Bundle with all the components needed to create an animation.
use bevy::prelude::*;

use super::{Animation, AnimationFrame, AnimationTimer};

/// Component to hold the `unique_id` of the entity sprite.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub struct EntitySpriteId {
    /// The `unique_id` of the entity sprite.
    pub unique_id: String,
}

/// Component to describe the entity's current movement information.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub struct AnimationStatus {
    /// True if the entity is facing left.
    pub facing_left: bool,
    /// True if the entity is walking
    pub walking: bool,
    /// True if the entity is running
    pub running: bool,
}

impl Default for AnimationStatus {
    fn default() -> Self {
        Self {
            facing_left: true,
            walking: false,
            running: false,
        }
    }
}

/// Bundle with all the components needed to create an animation.
#[derive(Debug, Clone, Bundle)]
pub struct AnimationBundle {
    /// The entity sprite `unique_id`.
    pub entity_sprite_id: EntitySpriteId,
    /// The current animation of the sprite.
    pub current_animation: Animation,
    /// The current frame of the animation.
    pub animation_frame: AnimationFrame,
    /// The timer that will be used to update the animation.
    pub animation_timer: AnimationTimer,
    /// The movement status of the entity.
    pub animation_status: AnimationStatus,
}

impl AnimationBundle {
    /// Create a new animation bundle.
    #[must_use]
    pub fn new(entity_sprite_id: &str) -> Self {
        let mut timer = AnimationTimer::new(1.0);
        timer.pause();
        Self {
            entity_sprite_id: EntitySpriteId {
                unique_id: entity_sprite_id.to_string(),
            },
            current_animation: Animation::Idle,
            animation_frame: AnimationFrame::new(),
            animation_timer: timer,
            animation_status: AnimationStatus::default(),
        }
    }
}
