//! System that handles the animation of entities with a `Sprite` component.
//!
//! This has to regard the `Animation` component and the `AnimationDefinition` component.
use bevy::prelude::*;

use crate::data_loader::storage::GameData;

use super::{
    animation_bundle::AnimationStatus, Animation, AnimationFrame, AnimationTimer, EntitySpriteId,
};

/// System that handles the drawing and animation of sprites.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn draw_sprites(
    time: Res<Time>,
    mut query: Query<(
        &mut TextureAtlas,
        &mut Sprite,
        &mut AnimationTimer,
        &mut AnimationFrame,
        &Animation,
        &EntitySpriteId,
        &AnimationStatus,
    )>,
    game_data: Res<GameData>,
) {
    // Loop over all the entities with the `AnimatedSpriteBundle` component (which should match our query)
    for (
        mut texture_atlas,
        mut sprite,
        mut timer,
        mut frame,
        &animation_state,
        sprite_id,
        status,
    ) in &mut query
    {
        // attempt to get the entity sprite
        let Some(entity_sprite) = game_data.entity_sprites.get(&sprite_id.unique_id) else {
            warn!("Entity sprite with id {} not found", sprite_id.unique_id);
            continue;
        };

        let current_animation = entity_sprite.get_animation(animation_state);

        // Check that the current animation is the same as the animation state.
        if animation_state != timer.animation() {
            // Update the timer with the new animation state and duration as needed.
            timer.update(animation_state, current_animation.frame_duration);
            // Reset the frame to 0.
            frame.reset();
        }

        // Advance the timer.
        if timer.advance(&time) {
            // Advance the frame.
            frame.next();

            // Get the current frame index.
            let frame_index = frame.get(current_animation.tile_indices.len());
            if let Some(texture_index) = current_animation.tile_indices.get(frame_index) {
                texture_atlas.index = *texture_index;
            } else {
                warn!(
                    "Frame index {} not found in animation {:?}",
                    frame_index, animation_state
                );
            }

            // Set the sprite's flip_x based on the facing direction.
            sprite.flip_x = status.facing_left;
        }
    }
}

/// Transitions the animation state based on the `AnimationStatus`. This automatically transitions from
/// an active state to idle when the action is no longer active.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn transition_to_idle(
    mut query: Query<
        (
            &mut Animation,
            &EntitySpriteId,
            &mut AnimationFrame,
            &AnimationStatus,
        ),
        Changed<AnimationFrame>,
    >,
    game_data: Res<GameData>,
) {
    for (mut animation, entity_sprite, mut frame, status) in &mut query {
        //todo: fix this

        // attempt to get the entity sprite
        let Some(entity_sprite) = game_data.entity_sprites.get(&entity_sprite.unique_id) else {
            warn!(
                "Entity sprite with id {} not found",
                entity_sprite.unique_id
            );
            continue;
        };

        let current_animation = entity_sprite.get_animation(*animation);

        // exit if the animation is not complete
        if !frame.is_fin(current_animation.tile_indices.len()) {
            continue;
        }

        // reset the frame
        frame.reset();

        // transition to idle
        match *animation {
            Animation::WalkAttack
            | Animation::Attack
            | Animation::Cast
            | Animation::RunAttack
            | Animation::WalkCast
            | Animation::RunCast => {
                if !status.walking && !status.running {
                    *animation = Animation::Idle;
                } else if status.walking {
                    *animation = Animation::Walk;
                } else if status.running {
                    *animation = Animation::Run;
                }
            }
            _ => {}
        }
    }
}
