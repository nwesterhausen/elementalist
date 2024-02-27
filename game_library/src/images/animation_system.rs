//! System that handles the animation of entities with a `Sprite` component.
//!
//! This has to regard the `Animation` component and the `AnimationDefinition` component.
use bevy::prelude::*;

use crate::data_loader::storage::GameData;

use super::{Animation, AnimationFrame, AnimationTimer, EntitySpriteId};

/// System that handles the animation of entities with the `AnimatedSpriteBundle` component.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn animate_sprites(
    time: Res<Time>,
    mut animation_query: Query<(
        &mut AnimationTimer,
        &mut AnimationFrame,
        &Animation,
        &EntitySpriteId,
    )>,
    game_data: Res<GameData>,
) {
    for (mut timer, mut frame, &animation_state, sprite_id) in &mut animation_query {
        let Some(entity_sprite) = game_data.entity_sprites.get(&sprite_id.unique_id) else {
            warn!("Entity sprite with id {} not found", sprite_id.unique_id);
            continue;
        };

        let current_animation = entity_sprite.get_animation(animation_state);

        // Check that the current animation is the same as the animation state.
        if animation_state != timer.animation() {
            // Update the timer with the new animation state and duration as needed.
            timer.update(animation_state, current_animation.frame_duration);

            // Update the frame with the new number of frames.
            frame.update_max(current_animation.tile_indices.len());
        }

        // Advance the timer.
        if timer.advance(&time) {
            // Advance the frame.
            frame.next();
        }
    }
}

/// System that handles drawing the current frame of the animated sprite.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn draw_animated_frames(
    mut query: Query<(
        &mut TextureAtlas,
        &AnimationFrame,
        &Animation,
        &EntitySpriteId,
    )>,
    game_data: Res<GameData>,
) {
    for (mut texture_atlas, frame, &animation_state, sprite_id) in &mut query {
        let Some(entity_sprite) = game_data.entity_sprites.get(&sprite_id.unique_id) else {
            warn!("Entity sprite with id {} not found", sprite_id.unique_id);
            continue;
        };

        let current_animation = entity_sprite.get_animation(animation_state);

        let frame_index = frame.get(current_animation.tile_indices.len());
        texture_atlas.index = frame_index;
    }
}
