//! Bundle with all the components needed to create an animation.
use bevy::prelude::*;

use super::{
    Animation, AnimationActionState, AnimationFrame, AnimationMovementState,
    AnimationReactionState, AnimationTimer,
};

/// Component to hold the `unique_id` of the entity sprite.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub struct EntitySpriteId {
    /// The `unique_id` of the entity sprite.
    pub unique_id: String,
}

/// Component to describe the entity's current movement information.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component)]
pub struct AnimationStatus {
    /// True if the entity is facing left.
    pub facing_left: bool,
    /// Movement status of the entity.
    pub movement: AnimationMovementState,
    /// Action status of the entity.
    pub action: AnimationActionState,
    /// Reaction status of the entity.
    pub reaction: AnimationReactionState,
}

/// Bundle with all the components needed to create an animation.
#[derive(Debug, Clone, Bundle)]
pub struct AnimationBundle {
    /// The entity sprite `unique_id`.
    pub entity_sprite_id: EntitySpriteId,
    /// The current frame of the animation.
    pub animation_frame: AnimationFrame,
    /// The timer that will be used to update the animation.
    pub animation_timer: AnimationTimer,
    /// The movement status of the entity.
    pub animation_status: AnimationStatus,
    /// The current combined animation state.
    pub animation: Animation,
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
            animation_frame: AnimationFrame::default(),
            animation_timer: AnimationTimer::default(),
            animation_status: AnimationStatus::default(),
            animation: Animation::default(),
        }
    }
}
