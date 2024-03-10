//! System that handles the animation of entities with a `Sprite` component.
//!
//! This has to regard the `Animation` component and the `AnimationDefinition` component.
use bevy::prelude::*;

use crate::data_loader::storage::GameData;

use super::{
    animation_bundle::AnimationStatus,
    state::{Action, Movement, Reaction},
    Animation, AnimationFrame, AnimationTimer, EntitySpriteId,
};

/// System that updates the `tile_atlas` texture based on the current animation state.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn update_tile_atlas_textures(
    mut query: Query<
        (
            &mut TextureAtlas,
            &EntitySpriteId,
            &Animation,
            &mut Handle<Image>,
        ),
        Changed<Animation>,
    >,
    game_data: Res<GameData>,
) {
    for (mut texture_atlas, sprite_id, animation_state, mut image) in &mut query {
        // attempt to get the entity sprite
        let Some(entity_sprite) = game_data.entity_sprites.get(&sprite_id.unique_id) else {
            warn!("Entity sprite with id {} not found", sprite_id.unique_id);
            continue;
        };

        let current_animation = entity_sprite.get_animation(*animation_state);

        // get the tile_atlas for the current animation
        let Some(tile_atlas) = game_data.tile_atlas.get(&current_animation.tileset) else {
            warn!("Tileset with id {} not found", current_animation.tileset);
            continue;
        };

        // update the texture atlas
        *image = tile_atlas.texture_handle.clone();
        texture_atlas.layout = tile_atlas.atlas_handle.clone();
    }
}

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
        &mut AnimationStatus,
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
        mut status,
    ) in &mut query
    {
        // attempt to get the entity sprite
        let Some(entity_sprite) = game_data.entity_sprites.get(&sprite_id.unique_id) else {
            warn!("Entity sprite with id {} not found", sprite_id.unique_id);
            continue;
        };

        let current_animation = entity_sprite.get_animation(animation_state);

        if timer.advance(&time) {
            // Advance the frame
            frame.next();

            // Get the current frame index and update the texture atlas
            let frame_index = frame.get(current_animation.tile_indices.len());
            if let Some(texture_index) = current_animation.tile_indices.get(frame_index) {
                texture_atlas.index = *texture_index;
            } else {
                warn!(
                    "Frame index {} not found in animation {:?}",
                    frame_index, animation_state
                );
            }

            // Per animation state, check if the animation is finished and reset the state if it is.
            if status.reaction != Reaction::None {
                // Check if the frame is at the end of the animation.
                if frame.is_fin(current_animation.tile_indices.len()) {
                    info!(
                        "Reaction animation finished on sprite_id {}",
                        sprite_id.unique_id
                    );
                    // Set the reaction to `Reaction::None`.
                    status.reaction = Reaction::None;
                }
            } else if status.action != Action::None {
                // Check if the frame is at the end of the animation.
                if frame.is_fin(current_animation.tile_indices.len()) {
                    info!(
                        "Action animation finished on sprite_id {}",
                        sprite_id.unique_id
                    );
                    // Reset the frame.
                    frame.reset();
                    // Set the action to `Action::None`.
                    status.action = Action::None;
                }
            }
        }

        // Check if the reaction is `Reaction::None`.

        // Set the sprite's flip_x based on the facing direction.
        sprite.flip_x = status.facing_left;
    }
}

/// Transitions the animation state based on the `AnimationStatus`. This automatically transitions from
/// an active state to idle when the action is no longer active.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn update_current_animation(
    mut query: Query<(
        &mut Animation,
        &AnimationStatus,
        &mut AnimationTimer,
        &mut AnimationFrame,
        &EntitySpriteId,
    )>,
    game_data: Res<GameData>,
) {
    for (mut animation, status, mut timer, mut frame, entity_sprite) in &mut query {
        // determine the current animation
        // this is based on the status, looking first at the reaction, then the action, and finally the movement
        let current_animation =
            get_current_animation(status.reaction, status.action, status.movement);

        // if the current animation is different from the previous one, update the animation
        if *animation != current_animation {
            info!(
                "Transitioning from {:?} to {:?}",
                *animation, current_animation
            );
            *animation = current_animation;

            // Get the entity sprite
            let Some(entity_sprite) = game_data.entity_sprites.get(&entity_sprite.unique_id) else {
                warn!(
                    "Entity sprite with id {} not found",
                    entity_sprite.unique_id
                );
                continue;
            };

            // Get the current animation definition
            let current_animation_definition = entity_sprite.get_animation(current_animation);

            // Reset the frame and timer
            frame.reset();
            timer.update(current_animation_definition.frame_duration);
        }
    }
}

/// Function which takes the three states and returns the current animation state.
/// This is based on the priority of reaction > movement + action > movement.
///
/// This is used to determine the final animation state.
///
/// ## Parameters
///
/// - `reaction` - The reaction status of the entity.
/// - `action` - The action status of the entity.
/// - `movement` - The movement status of the entity.
///
/// ## Returns
///
/// The current animation state.
#[must_use]
const fn get_current_animation(
    reaction: Reaction,
    action: Action,
    movement: Movement,
) -> Animation {
    match reaction {
        Reaction::None => match action {
            Action::None => match movement {
                Movement::Idle => Animation::Idle,
                Movement::Walk => Animation::Walk,
                Movement::Run => Animation::Run,
                Movement::Jump => Animation::Jump,
            },
            Action::Cast => match movement {
                Movement::Walk => Animation::WalkCast,
                Movement::Run => Animation::RunCast,
                _ => Animation::Cast,
            },
            Action::Attack => match movement {
                Movement::Walk => Animation::WalkAttack,
                Movement::Run => Animation::RunAttack,
                _ => Animation::Attack,
            },
        },
        Reaction::Hurt => Animation::Hurt,
        Reaction::Stun => Animation::Stun,
        Reaction::Dead => Animation::Dead,
        Reaction::Death => Animation::Death,
    }
}
